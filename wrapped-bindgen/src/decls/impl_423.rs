macro_rules! deps {
    () => {
        Blob!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl AttributeType { pub fn parent (& self) -> MemberRefParent { match self { Self :: MethodDef (row) => row . parent () , Self :: MemberRef (row) => row . parent () , } } pub fn signature (& self) -> Blob { match self { Self :: MethodDef (row) => row . blob (4) , Self :: MemberRef (row) => row . blob (2) , } } }
    };
}

impl_423!();