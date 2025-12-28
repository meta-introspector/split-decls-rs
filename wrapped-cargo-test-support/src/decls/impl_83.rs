macro_rules! deps {
    () => {
        CargoPathExt!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl CargoPathExt for PathBuf { fn to_url (& self) -> url :: Url { self . as_path () . to_url () } fn rm_rf (& self) { self . as_path () . rm_rf () } fn mkdir_p (& self) { self . as_path () . mkdir_p () } fn ls_r (& self) -> Vec < PathBuf > { self . as_path () . ls_r () } fn move_in_time < F > (& self , travel_amount : F) where F : Fn (i64 , u32) -> (i64 , u32) , { self . as_path () . move_in_time (travel_amount) } # [track_caller] fn assert_build_dir_layout (& self , expected : impl snapbox :: IntoData) { self . as_path () . assert_build_dir_layout (expected) ; } # [track_caller] fn assert_dir_layout (& self , expected : impl snapbox :: IntoData , ignored_path_patterns : & [String] ,) { self . as_path () . assert_dir_layout (expected , ignored_path_patterns) ; } }
    };
}

impl_83!();