macro_rules! deps {
    () => {
        UnEither!();
        Folder!();
        UnzipOp!();
    };
}

macro_rules! impl_961 {
    () => {
        deps!();
        impl < L , R > UnzipOp < Either < L , R > > for UnEither where L : Send , R : Send , { type Left = L ; type Right = R ; fn consume < FL , FR > (& self , item : Either < L , R > , left : FL , right : FR) -> (FL , FR) where FL : Folder < L > , FR : Folder < R > , { match item { Either :: Left (item) => (left . consume (item) , right) , Either :: Right (item) => (left , right . consume (item)) , } } }
    };
}

impl_961!();