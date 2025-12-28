macro_rules! deps {
    () => {
        CStrLike!();
        Error!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl CStrLike for & String { type Baked = CString ; type Error = std :: ffi :: NulError ; fn bake (self) -> Result < Self :: Baked , Self :: Error > { CString :: new (self . as_bytes ()) } fn into_c_string (self) -> Result < CString , Self :: Error > { CString :: new (self . as_bytes ()) } }
    };
}

impl_9!();