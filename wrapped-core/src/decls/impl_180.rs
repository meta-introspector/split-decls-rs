macro_rules! deps {
    () => {
        CanInto!();
        ParamValue!();
        Param!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < T , U > Param < T , CopyType > for U where T : TypeKind < TypeKind = CopyType > + Clone , U : TypeKind < TypeKind = CopyType > + Clone , U : imp :: CanInto < T > , { unsafe fn param (self) -> ParamValue < T > { unsafe { ParamValue :: Owned (transmute_copy (& self)) } } }
    };
}

impl_180!();