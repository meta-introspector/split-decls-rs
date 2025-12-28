macro_rules! deps {
    () => {
        UnindexedConsumer!();
        Folder!();
        Consumer!();
        Reducer!();
        FlatMapIterConsumer!();
        FlatMapIterFolder!();
    };
}

macro_rules! impl_525 {
    () => {
        deps!();
        impl < 'f , T , U , C , F > Consumer < T > for FlatMapIterConsumer < 'f , C , F > where C : UnindexedConsumer < U :: Item > , F : Fn (T) -> U + Sync , U : IntoIterator , { type Folder = FlatMapIterFolder < 'f , C :: Folder , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , C :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (FlatMapIterConsumer :: new (left , self . map_op) , FlatMapIterConsumer :: new (right , self . map_op) , reducer ,) } fn into_folder (self) -> Self :: Folder { FlatMapIterFolder { base : self . base . into_folder () , map_op : self . map_op , } } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_525!()