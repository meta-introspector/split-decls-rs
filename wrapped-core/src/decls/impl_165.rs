macro_rules! deps {
    () => {
        OutRef!();
        OutParam!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < T > OutParam < T , CloneType > for & mut T where T : TypeKind < TypeKind = CloneType > + Clone + Default , { unsafe fn borrow_mut (& self) -> OutRef < '_ , T > { unsafe { let this : & mut T = transmute_copy (self) ; take (this) ; transmute_copy (self) } } }
    };
}

impl_165!();