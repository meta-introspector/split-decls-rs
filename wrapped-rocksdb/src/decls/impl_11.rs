macro_rules! deps {
    () => {
        Error!();
        CStrLike!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl CStrLike for CString { type Baked = CString ; type Error = std :: convert :: Infallible ; fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (self) } fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self) } }
    };
}

impl_11!();