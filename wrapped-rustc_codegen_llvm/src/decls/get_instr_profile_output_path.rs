macro_rules! get_instr_profile_output_path {
    () => {
        fn get_instr_profile_output_path (config : & ModuleConfig) -> Option < CString > { config . instrument_coverage . then (| | c"default_%m_%p.profraw" . to_owned ()) }
    };
}

get_instr_profile_output_path!()