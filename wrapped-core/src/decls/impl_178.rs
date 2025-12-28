macro_rules! deps {
    () => {
        CanInto!();
        ParamValue!();
        Param!();
        Interface!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < T , U > Param < T , InterfaceType > for & U where T : TypeKind < TypeKind = InterfaceType > + Clone , T : Interface , U : Interface , U : imp :: CanInto < T > , { unsafe fn param (self) -> ParamValue < T > { unsafe { if U :: QUERY { self . cast () . map_or (ParamValue :: Borrowed (zeroed ()) , | ok | ParamValue :: Owned (ok)) } else { ParamValue :: Borrowed (transmute_copy (self)) } } } }
    };
}

impl_178!();