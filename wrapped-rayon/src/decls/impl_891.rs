macro_rules! deps {
    () => {
        Consumer!();
        TakeAnyWhileConsumer!();
        TakeAnyWhileFolder!();
        Folder!();
        Reducer!();
    };
}

macro_rules! impl_891 {
    () => {
        deps!();
        impl < 'p , T , C , P > Consumer < T > for TakeAnyWhileConsumer < 'p , C , P > where C : Consumer < T > , P : Fn (& T) -> bool + Sync , { type Folder = TakeAnyWhileFolder < 'p , C :: Folder , P > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (TakeAnyWhileConsumer { base : left , .. self } , TakeAnyWhileConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { TakeAnyWhileFolder { base : self . base . into_folder () , predicate : self . predicate , taking : self . taking , } } fn full (& self) -> bool { ! self . taking . load (Ordering :: Relaxed) || self . base . full () } }
    };
}

impl_891!();