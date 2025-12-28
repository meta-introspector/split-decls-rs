macro_rules! deps {
    () => {
        Folder!();
        Partition!();
        UnzipOp!();
    };
}

macro_rules! impl_943 {
    () => {
        deps!();
        impl < P , T > UnzipOp < T > for Partition < P > where P : Fn (& T) -> bool + Sync + Send , T : Send , { type Left = T ; type Right = T ; fn consume < FA , FB > (& self , item : T , left : FA , right : FB) -> (FA , FB) where FA : Folder < T > , FB : Folder < T > , { if (self . predicate) (& item) { (left . consume (item) , right) } else { (left , right . consume (item)) } } }
    };
}

impl_943!();