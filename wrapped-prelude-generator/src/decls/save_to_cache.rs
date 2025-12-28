macro_rules! deps {
    () => {
        FileAnalysisCache!();
    };
}

macro_rules! save_to_cache {
    () => {
        deps!();
        async fn save_to_cache (cache_file_path : & Path , data : & FileAnalysisCache) -> anyhow :: Result < () > { tokio :: fs :: create_dir_all (cache_file_path . parent () . unwrap ()) . await ? ; let serialized_data = serde_json :: to_string_pretty (data) ? ; tokio :: fs :: write (cache_file_path , serialized_data) . await ? ; Ok (()) }
    };
}

save_to_cache!()