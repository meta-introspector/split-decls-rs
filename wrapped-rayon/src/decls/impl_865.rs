macro_rules! deps {
    () => {
        SumFolder!();
        Reducer!();
        Consumer!();
        Folder!();
        SumConsumer!();
    };
}

macro_rules! impl_865 {
    () => {
        deps!();
        impl < S , T > Consumer < T > for SumConsumer < S > where S : Send + Sum < T > + Sum , { type Folder = SumFolder < S > ; type Reducer = Self ; type Result = S ; fn split_at (self , _index : usize) -> (Self , Self , Self) { (SumConsumer :: new () , SumConsumer :: new () , SumConsumer :: new ()) } fn into_folder (self) -> Self :: Folder { SumFolder { sum : iter :: empty :: < T > () . sum () , } } fn full (& self) -> bool { false } }
    };
}

impl_865!();