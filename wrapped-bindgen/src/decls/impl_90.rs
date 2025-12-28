macro_rules! impl_90 {
    () => {
        impl MemberRef { pub fn parent (& self) -> MemberRefParent { self . decode (0) } }
    };
}

impl_90!()