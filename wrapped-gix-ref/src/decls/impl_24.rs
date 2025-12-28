macro_rules! deps {
    () => {
        PartialNameRef!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl PartialNameRef { # [doc = " Convert this name into the relative path possibly identifying the reference location."] # [doc = " Note that it may be only a partial path though."] pub fn to_partial_path (& self) -> & Path { gix_path :: from_byte_slice (self . 0 . as_bstr ()) } # [doc = " Provide the name as binary string which is known to be a valid partial ref name."] pub fn as_bstr (& self) -> & BStr { & self . 0 } }
    };
}

impl_24!();