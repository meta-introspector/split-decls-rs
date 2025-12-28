macro_rules! deps {
    () => {
        CStrLike!();
        Error!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl CStrLike for & str { type Baked = CString ; type Error = std :: ffi :: NulError ; fn bake (self) -> Result < Self :: Baked , Self :: Error > { CString :: new (self) } fn into_c_string (self) -> Result < CString , Self :: Error > { CString :: new (self) } }
    };
}

impl_8!()