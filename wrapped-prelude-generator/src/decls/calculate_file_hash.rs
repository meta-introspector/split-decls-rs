macro_rules! calculate_file_hash {
    () => {
        fn calculate_file_hash (file_path : & Path) -> anyhow :: Result < String > { let mut file = fs :: File :: open (file_path) ? ; let mut hasher = Sha256 :: new () ; std :: io :: copy (& mut file , & mut hasher) ? ; Ok (format ! ("{:x}" , hasher . finalize ())) }
    };
}

calculate_file_hash!()