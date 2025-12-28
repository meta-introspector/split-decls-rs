macro_rules! deps {
    () => {
        OutParam!();
        OutRef!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < T > OutParam < T , CopyType > for & mut T where T : TypeKind < TypeKind = CopyType > + Clone + Default , { unsafe fn borrow_mut (& self) -> OutRef < '_ , T > { unsafe { transmute_copy (self) } } }
    };
}

impl_166!()