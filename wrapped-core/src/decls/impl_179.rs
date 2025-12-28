macro_rules! deps {
    () => {
        Param!();
        ParamValue!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < T > Param < T , CloneType > for & T where T : TypeKind < TypeKind = CloneType > + Clone , { unsafe fn param (self) -> ParamValue < T > { unsafe { ParamValue :: Borrowed (transmute_copy (self)) } } }
    };
}

impl_179!()