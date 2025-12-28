macro_rules! deps {
    () => {
        FlattenIterConsumer!();
        Consumer!();
        Folder!();
        UnindexedConsumer!();
        Reducer!();
        FlattenIterFolder!();
    };
}

macro_rules! impl_545 {
    () => {
        deps!();
        impl < T , C > Consumer < T > for FlattenIterConsumer < C > where C : UnindexedConsumer < T :: Item > , T : IntoIterator , { type Folder = FlattenIterFolder < C :: Folder > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , C :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (FlattenIterConsumer :: new (left) , FlattenIterConsumer :: new (right) , reducer ,) } fn into_folder (self) -> Self :: Folder { FlattenIterFolder { base : self . base . into_folder () , } } fn full (& self) -> bool { self . base . full () } }
    };
}

impl_545!()