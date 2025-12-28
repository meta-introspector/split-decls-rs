macro_rules! deps {
    () => {
        UnindexedConsumer!();
        Consumer!();
        FlatMapConsumer!();
        Reducer!();
        FlatMapFolder!();
        Folder!();
        IntoParallelIterator!();
    };
}

macro_rules! impl_514 {
    () => {
        deps!();
        impl < 'f , T , U , C , F > Consumer < T > for FlatMapConsumer < 'f , C , F > where C : UnindexedConsumer < U :: Item > , F : Fn (T) -> U + Sync , U : IntoParallelIterator , { type Folder = FlatMapFolder < 'f , C , F , C :: Result > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , C :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (FlatMapConsumer :: new (left , self . map_op) , FlatMapConsumer :: new (right , self . map_op) , reducer ,) } fn into_folder (self) -> Self :: Folder { FlatMapFolder { base : self . base , map_op : self . map_op , previous : None , } } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_514!()