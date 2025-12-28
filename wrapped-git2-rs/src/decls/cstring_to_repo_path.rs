macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! cstring_to_repo_path {
    () => {
        deps!();
        pub fn cstring_to_repo_path < T : IntoCString > (path : T) -> Result < CString , Error > { fixup_windows_path (path . into_c_string () ?) }
    };
}

cstring_to_repo_path!()