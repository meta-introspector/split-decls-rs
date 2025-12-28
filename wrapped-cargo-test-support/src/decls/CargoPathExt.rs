macro_rules! CargoPathExt {
    () => {
        # [doc = " Common path and file operations"] pub trait CargoPathExt { fn to_url (& self) -> url :: Url ; fn rm_rf (& self) ; fn mkdir_p (& self) ; # [doc = " Returns a list of all files and directories underneath the given"] # [doc = " directory, recursively, including the starting path."] fn ls_r (& self) -> Vec < PathBuf > ; fn move_into_the_past (& self) { self . move_in_time (| sec , nsec | (sec - 3600 , nsec)) } fn move_into_the_future (& self) { self . move_in_time (| sec , nsec | (sec + 3600 , nsec)) } fn move_in_time < F > (& self , travel_amount : F) where F : Fn (i64 , u32) -> (i64 , u32) ; fn assert_build_dir_layout (& self , expected : impl snapbox :: IntoData) ; fn assert_dir_layout (& self , expected : impl snapbox :: IntoData , ignored_path_patterns : & [String]) ; }
    };
}

CargoPathExt!();