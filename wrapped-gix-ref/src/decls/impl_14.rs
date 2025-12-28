macro_rules! deps {
    () => {
        FullNameRef!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl FullNameRef { # [doc = " Return the file name portion of a full name, for instance `main` if the"] # [doc = " full name was `refs/heads/main`."] pub fn file_name (& self) -> & BStr { self . 0 . rsplitn (2 , | b | * b == b'/') . next () . expect ("valid ref") . as_bstr () } }
    };
}

impl_14!();