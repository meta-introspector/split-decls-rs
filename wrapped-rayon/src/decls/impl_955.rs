macro_rules! deps {
    () => {
        Folder!();
        UnzipOp!();
        UnzipFolder!();
    };
}

macro_rules! impl_955 {
    () => {
        deps!();
        impl < 'a , T , OP , FA , FB > Folder < T > for UnzipFolder < 'a , OP , FA , FB > where OP : UnzipOp < T > , FA : Folder < OP :: Left > , FB : Folder < OP :: Right > , { type Result = (FA :: Result , FB :: Result) ; fn consume (self , item : T) -> Self { let (left , right) = self . op . consume (item , self . left , self . right) ; UnzipFolder { op : self . op , left , right , } } fn complete (self) -> Self :: Result { (self . left . complete () , self . right . complete ()) } fn full (& self) -> bool { self . left . full () && self . right . full () } }
    };
}

impl_955!()