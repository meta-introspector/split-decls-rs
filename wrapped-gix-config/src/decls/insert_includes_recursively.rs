macro_rules! deps {
    () => {
        SectionId!();
        File!();
        Error!();
        Options!();
        Metadata!();
    };
}

macro_rules! insert_includes_recursively {
    () => {
        deps!();
        fn insert_includes_recursively (section_ids_and_include_paths : Vec < (SectionId , crate :: Path < '_ >) > , target_config : & mut File < 'static > , depth : u8 , options : init :: Options < '_ > , buf : & mut Vec < u8 > ,) -> Result < () , Error > { for (section_id , config_path) in section_ids_and_include_paths { let meta = OwnShared :: clone (& target_config . sections [& section_id] . meta) ; let target_config_path = meta . path . as_deref () ; let config_path = match resolve_path (config_path , target_config_path , options . includes) ? { Some (p) => p , None => continue , } ; if ! config_path . is_file () { continue ; } buf . clear () ; std :: io :: copy (& mut std :: fs :: File :: open (& config_path) . map_err (| err | Error :: Io { source : err , path : config_path . to_owned () , }) ? , buf ,) . map_err (Error :: CopyBuffer) ? ; let config_meta = Metadata { path : Some (config_path) , trust : meta . trust , level : meta . level + 1 , source : meta . source , } ; let no_follow_options = init :: Options { includes : includes :: Options :: no_follow () , .. options } ; let mut include_config = File :: from_bytes_owned (buf , config_meta , no_follow_options) . map_err (| err | match err { init :: Error :: Parse (err) => Error :: Parse (err) , init :: Error :: Interpolate (err) => Error :: Interpolate (err) , init :: Error :: Includes (_) => unreachable ! ("BUG: {:?} not possible due to no-follow options" , err) , }) ? ; resolve_includes_recursive (Some (target_config) , & mut include_config , depth + 1 , buf , options) ? ; target_config . append_or_insert (include_config , Some (section_id)) ; } Ok (()) }
    };
}

insert_includes_recursively!();