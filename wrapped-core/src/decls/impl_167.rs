macro_rules! deps {
    () => {
        OutParam!();
        OutRef!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < T > OutParam < T , InterfaceType > for & mut Option < T > where T : TypeKind < TypeKind = InterfaceType > + Clone , { unsafe fn borrow_mut (& self) -> OutRef < '_ , T > { unsafe { let this : & mut Option < T > = transmute_copy (self) ; take (this) ; transmute_copy (self) } } }
    };
}

impl_167!()