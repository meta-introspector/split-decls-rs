macro_rules! deps {
    () => {
        Folder!();
        WhileSomeConsumer!();
        Consumer!();
        WhileSomeFolder!();
        Reducer!();
    };
}

macro_rules! impl_1007 {
    () => {
        deps!();
        impl < 'f , T , C > Consumer < Option < T > > for WhileSomeConsumer < 'f , C > where C : Consumer < T > , T : Send , { type Folder = WhileSomeFolder < 'f , C :: Folder > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (WhileSomeConsumer { base : left , .. self } , WhileSomeConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { WhileSomeFolder { base : self . base . into_folder () , full : self . full , } } fn full (& self) -> bool { self . full . load (Ordering :: Relaxed) || self . base . full () } }
    };
}

impl_1007!()