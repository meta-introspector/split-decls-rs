macro_rules! deps {
    () => {
        Error!();
        CStrLike!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl CStrLike for & CStr { type Baked = Self ; type Error = std :: convert :: Infallible ; fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (self) } fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self . to_owned ()) } }
    };
}

impl_10!()