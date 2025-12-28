macro_rules! deps {
    () => {
        OutRef!();
        OutParam!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < T > OutParam < T > for Option < & mut T > where T : Type < T > , { unsafe fn borrow_mut (& self) -> OutRef < '_ , T > { unsafe { match self { Some (this) => transmute_copy (this) , None => zeroed () , } } } }
    };
}

impl_168!()