macro_rules! generate_session_dir_path {
    () => {
        # [doc = " Generates unique directory path of the form:"] # [doc = " {crate_dir}/s-{timestamp}-{random-number}-working"] fn generate_session_dir_path (crate_dir : & Path) -> PathBuf { let timestamp = timestamp_to_string (SystemTime :: now ()) ; debug ! ("generate_session_dir_path: timestamp = {}" , timestamp) ; let random_number = rng () . next_u32 () ; debug ! ("generate_session_dir_path: random_number = {}" , random_number) ; let (zeroes , timestamp) = timestamp . split_at (3) ; assert_eq ! (zeroes , "000") ; let directory_name = format ! ("s-{}-{}-working" , timestamp , random_number . to_base_fixed_len (CASE_INSENSITIVE)) ; debug ! ("generate_session_dir_path: directory_name = {}" , directory_name) ; let directory_path = crate_dir . join (directory_name) ; debug ! ("generate_session_dir_path: directory_path = {}" , directory_path . display ()) ; directory_path }
    };
}

generate_session_dir_path!()