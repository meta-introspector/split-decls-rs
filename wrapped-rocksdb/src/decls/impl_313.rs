macro_rules! deps {
    () => {
        CStrLike!();
        PropName!();
        Error!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl < 'a > CStrLike for & 'a PropName { type Baked = & 'a CStr ; type Error = std :: convert :: Infallible ; # [inline] fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (& self . 0) } # [inline] fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self . 0 . to_owned ()) } }
    };
}

impl_313!();