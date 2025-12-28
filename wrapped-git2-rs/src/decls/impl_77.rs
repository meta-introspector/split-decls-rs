macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl IntoCString for OsString { # [cfg (unix)] fn into_c_string (self) -> Result < CString , Error > { use std :: os :: unix :: prelude :: * ; let s : & OsStr = self . as_ref () ; Ok (CString :: new (s . as_bytes ()) ?) } # [cfg (windows)] fn into_c_string (self) -> Result < CString , Error > { match self . to_str () { Some (s) => s . into_c_string () , None => Err (Error :: from_str ("only valid unicode paths are accepted on windows" ,)) , } } }
    };
}

impl_77!();