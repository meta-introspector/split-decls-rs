macro_rules! deps {
    () => {
        File!();
        Error!();
        Options!();
    };
}

macro_rules! resolve_includes_recursive {
    () => {
        deps!();
        fn resolve_includes_recursive (search_config : Option < & File < 'static > > , target_config : & mut File < 'static > , depth : u8 , buf : & mut Vec < u8 > , options : init :: Options < '_ > ,) -> Result < () , Error > { if depth == options . includes . max_depth { return if options . includes . err_on_max_depth_exceeded { Err (Error :: IncludeDepthExceeded { max_depth : options . includes . max_depth , }) } else { Ok (()) } ; } for id in target_config . section_order . clone () . into_iter () { let section = & target_config . sections [& id] ; let header = & section . header ; let header_name = header . name . as_ref () ; let mut paths = None ; if header_name == "include" && header . subsection_name . is_none () { paths = Some (gather_paths (section , id)) ; } else if header_name == "includeIf" { if let Some (condition) = & header . subsection_name { let target_config_path = section . meta . path . as_deref () ; if include_condition_match (condition . as_ref () , target_config_path , search_config . unwrap_or (target_config) , options . includes ,) ? { paths = Some (gather_paths (section , id)) ; } } } if let Some (paths) = paths { insert_includes_recursively (paths , target_config , depth , options , buf) ? ; } } Ok (()) }
    };
}

resolve_includes_recursive!()