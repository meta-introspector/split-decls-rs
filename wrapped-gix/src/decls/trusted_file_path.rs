macro_rules! deps {
    () => {
        Path!();
        Environment!();
        Error!();
    };
}

macro_rules! trusted_file_path {
    () => {
        deps!();
        pub (crate) fn trusted_file_path < 'config > (config : & 'config gix_config :: File < '_ > , key : impl gix_config :: AsKey , filter : impl FnMut (& Metadata) -> bool , lenient_config : bool , environment : crate :: open :: permissions :: Environment ,) -> Option < Result < Cow < 'config , std :: path :: Path > , gix_config :: path :: interpolate :: Error > > { let path = config . path_filter (& key , filter) ? ; if lenient_config && path . is_empty () { let _key = key . as_key () ; gix_trace :: info ! ("Ignored empty path at {section_name}.{subsection_name:?}.{name} due to lenient configuration" , section_name = _key . section_name , subsection_name = _key . subsection_name , name = _key . value_name) ; return None ; } let install_dir = crate :: path :: install_dir () . ok () ; let home = home_dir (environment) ; let ctx = config :: cache :: interpolate_context (install_dir . as_deref () , home . as_deref ()) ; Some (path . interpolate (ctx)) }
    };
}

trusted_file_path!();