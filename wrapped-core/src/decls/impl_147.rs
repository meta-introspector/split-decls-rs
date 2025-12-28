macro_rules! deps {
    () => {
        IInspectable!();
        HSTRING!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl IInspectable { # [doc = " Returns the canonical type name for the underlying object."] # [cfg (windows)] pub fn GetRuntimeClassName (& self) -> Result < HSTRING > { unsafe { let mut abi = null_mut () ; (self . vtable () . GetRuntimeClassName) (core :: mem :: transmute_copy (self) , & mut abi) . ok () ? ; Ok (core :: mem :: transmute :: < * mut c_void , HSTRING > (abi)) } } # [doc = " Gets the trust level of the current object."] pub fn GetTrustLevel (& self) -> Result < i32 > { unsafe { let mut value = 0 ; (self . vtable () . GetTrustLevel) (core :: mem :: transmute_copy (self) , & mut value) . ok () ? ; Ok (value) } } }
    };
}

impl_147!();