macro_rules! deps {
    () => {
        PropertyName!();
        CStrLike!();
        Error!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl < 'a > CStrLike for & 'a PropertyName { type Baked = & 'a CStr ; type Error = std :: convert :: Infallible ; # [inline] fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (self . as_c_str ()) } # [inline] fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self . 0 . clone ()) } }
    };
}

impl_327!()