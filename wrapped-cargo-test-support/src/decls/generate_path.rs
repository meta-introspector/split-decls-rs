macro_rules! generate_path {
    () => {
        fn generate_path (name : & str) -> PathBuf { paths :: root () . join (name) }
    };
}

generate_path!()