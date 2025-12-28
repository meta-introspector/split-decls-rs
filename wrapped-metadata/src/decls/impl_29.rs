macro_rules! deps {
    () => {
        Type!();
        Signature!();
        MethodDef!();
        MemberRef!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'a > AttributeType < 'a > { pub fn parent (& self) -> MemberRefParent < 'a > { match self { Self :: MethodDef (row) => row . parent () , Self :: MemberRef (row) => row . parent () , } } pub fn signature (& self , generics : & [Type]) -> Signature { match self { Self :: MethodDef (row) => row . signature (generics) , Self :: MemberRef (row) => row . signature (generics) , } } }
    };
}

impl_29!();