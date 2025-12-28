macro_rules! deps {
    () => {
        Consumer!();
        Folder!();
        TakeAnyConsumer!();
        TakeAnyFolder!();
        Reducer!();
    };
}

macro_rules! impl_880 {
    () => {
        deps!();
        impl < 'f , T , C > Consumer < T > for TakeAnyConsumer < 'f , C > where C : Consumer < T > , T : Send , { type Folder = TakeAnyFolder < 'f , C :: Folder > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (TakeAnyConsumer { base : left , .. self } , TakeAnyConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { TakeAnyFolder { base : self . base . into_folder () , count : self . count , } } fn full (& self) -> bool { self . count . load (Ordering :: Relaxed) == 0 || self . base . full () } }
    };
}

impl_880!();