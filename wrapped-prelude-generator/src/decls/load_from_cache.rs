macro_rules! deps {
    () => {
        FileAnalysisCache!();
    };
}

macro_rules! load_from_cache {
    () => {
        deps!();
        async fn load_from_cache (cache_file_path : & Path) -> anyhow :: Result < Option < FileAnalysisCache > > { if cache_file_path . exists () { let cached_content = tokio :: fs :: read_to_string (cache_file_path) . await ? ; let cached_data : FileAnalysisCache = serde_json :: from_str (& cached_content) ? ; Ok (Some (cached_data)) } else { Ok (None) } }
    };
}

load_from_cache!();