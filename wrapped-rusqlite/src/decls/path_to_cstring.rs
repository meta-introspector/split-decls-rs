macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! path_to_cstring {
    () => {
        deps!();
        # [cfg (not (unix))] fn path_to_cstring (p : & Path) -> Result < CString > { let s = p . to_str () . ok_or_else (| | Error :: InvalidPath (p . to_owned ())) ? ; Ok (CString :: new (s) ?) }
    };
}

path_to_cstring!();