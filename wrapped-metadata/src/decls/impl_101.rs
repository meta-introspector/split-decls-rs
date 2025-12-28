macro_rules! deps {
    () => {
        MemberRef!();
        Type!();
        Signature!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < 'a > MemberRef < 'a > { pub fn parent (& self) -> MemberRefParent < 'a > { self . decode (0) } pub fn name (& self) -> & 'a str { self . str (1) } pub fn signature (& self , generics : & [Type]) -> Signature { self . blob (2) . read_method_signature (generics) } }
    };
}

impl_101!()