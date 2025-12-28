macro_rules! deps {
    () => {
        Error!();
        CStrLike!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < 'a > CStrLike for & 'a CString { type Baked = & 'a CStr ; type Error = std :: convert :: Infallible ; fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (self) } fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self . clone ()) } }
    };
}

impl_12!()