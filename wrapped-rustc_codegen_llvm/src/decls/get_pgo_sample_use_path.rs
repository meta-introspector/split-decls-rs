macro_rules! get_pgo_sample_use_path {
    () => {
        fn get_pgo_sample_use_path (config : & ModuleConfig) -> Option < CString > { config . pgo_sample_use . as_ref () . map (| path_buf | CString :: new (path_buf . to_string_lossy () . as_bytes ()) . unwrap ()) }
    };
}

get_pgo_sample_use_path!();