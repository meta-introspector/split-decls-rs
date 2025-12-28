macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl oid { # [doc = " Verify that `self` matches the `expected` object ID."] # [doc = ""] # [doc = " Returns an [`Error`] containing both object IDs if they differ."] # [inline] pub fn verify (& self , expected : & oid) -> Result < () , Error > { if self == expected { Ok (()) } else { Err (Error { actual : self . to_owned () , expected : expected . to_owned () , }) } } }
    };
}

impl_32!();