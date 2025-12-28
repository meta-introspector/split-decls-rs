macro_rules! deps {
    () => {
        ParamValue!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < T : Type < T > > ParamValue < T > { pub fn abi (& self) -> T :: Abi { unsafe { match self { Self :: Owned (item) => transmute_copy (item) , Self :: Borrowed (borrowed) => transmute_copy (borrowed) , } } } pub fn borrow (& self) -> Ref < '_ , T > { unsafe { transmute_copy (& self . abi ()) } } }
    };
}

impl_183!()