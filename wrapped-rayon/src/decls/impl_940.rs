macro_rules! deps {
    () => {
        UnzipOp!();
        Folder!();
        Unzip!();
    };
}

macro_rules! impl_940 {
    () => {
        deps!();
        impl < A : Send , B : Send > UnzipOp < (A , B) > for Unzip { type Left = A ; type Right = B ; fn consume < FA , FB > (& self , item : (A , B) , left : FA , right : FB) -> (FA , FB) where FA : Folder < A > , FB : Folder < B > , { (left . consume (item . 0) , right . consume (item . 1)) } fn indexable () -> bool { true } }
    };
}

impl_940!()