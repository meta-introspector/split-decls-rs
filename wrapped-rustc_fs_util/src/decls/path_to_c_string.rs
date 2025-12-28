macro_rules! path_to_c_string {
    () => {
        # [cfg (windows)] pub fn path_to_c_string (p : & Path) -> CString { CString :: new (p . to_str () . unwrap ()) . unwrap () }
    };
}

path_to_c_string!()