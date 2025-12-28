macro_rules! deps {
    () => {
        FoldFolder!();
        Folder!();
    };
}

macro_rules! impl_558 {
    () => {
        deps!();
        impl < 'r , C , ID , F , T > Folder < T > for FoldFolder < 'r , C , ID , F > where C : Folder < ID > , F : Fn (ID , T) -> ID + Sync , { type Result = C :: Result ; fn consume (self , item : T) -> Self { let item = (self . fold_op) (self . item , item) ; FoldFolder { base : self . base , fold_op : self . fold_op , item , } } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { fn not_full < C , ID , T > (base : & C) -> impl Fn (& T) -> bool + '_ where C : Folder < ID > , { move | _ | ! base . full () } let base = self . base ; let item = iter . into_iter () . take_while (not_full (& base)) . fold (self . item , self . fold_op) ; FoldFolder { base , item , fold_op : self . fold_op , } } fn complete (self) -> C :: Result { self . base . consume (self . item) . complete () } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_558!();