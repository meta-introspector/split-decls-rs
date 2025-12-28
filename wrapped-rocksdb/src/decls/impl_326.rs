macro_rules! deps {
    () => {
        Error!();
        PropertyName!();
        CStrLike!();
    };
}

macro_rules! impl_326 {
    () => {
        deps!();
        impl CStrLike for PropertyName { type Baked = CString ; type Error = std :: convert :: Infallible ; # [inline] fn bake (self) -> Result < Self :: Baked , Self :: Error > { Ok (self . 0) } # [inline] fn into_c_string (self) -> Result < CString , Self :: Error > { Ok (self . 0) } }
    };
}

impl_326!();