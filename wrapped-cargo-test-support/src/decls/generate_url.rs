macro_rules! generate_url {
    () => {
        fn generate_url (name : & str) -> Url { Url :: from_file_path (generate_path (name)) . ok () . unwrap () }
    };
}

generate_url!()