macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! to_cpath {
    () => {
        deps!();
        pub (crate) fn to_cpath < P : AsRef < Path > > (path : P) -> Result < CString , Error > { match CString :: new (path . as_ref () . to_string_lossy () . as_bytes ()) { Ok (c) => Ok (c) , Err (e) => Err (Error :: new (format ! ("Failed to convert path to CString: {e}"))) , } }
    };
}

to_cpath!()