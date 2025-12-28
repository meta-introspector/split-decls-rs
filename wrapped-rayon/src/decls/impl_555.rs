macro_rules! deps {
    () => {
        FoldFolder!();
        Consumer!();
        Folder!();
        FoldConsumer!();
        Reducer!();
    };
}

macro_rules! impl_555 {
    () => {
        deps!();
        impl < 'r , U , T , C , ID , F > Consumer < T > for FoldConsumer < 'r , C , ID , F > where C : Consumer < U > , F : Fn (U , T) -> U + Sync , ID : Fn () -> U + Sync , U : Send , { type Folder = FoldFolder < 'r , C :: Folder , U , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (FoldConsumer { base : left , .. self } , FoldConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { FoldFolder { base : self . base . into_folder () , item : (self . identity) () , fold_op : self . fold_op , } } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_555!();