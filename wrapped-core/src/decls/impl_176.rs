macro_rules! deps {
    () => {
        Param!();
        ParamValue!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < T > Param < T > for Option < & T > where T : Type < T > , { unsafe fn param (self) -> ParamValue < T > { unsafe { ParamValue :: Borrowed (match self { Some (item) => transmute_copy (item) , None => zeroed () , }) } } }
    };
}

impl_176!();