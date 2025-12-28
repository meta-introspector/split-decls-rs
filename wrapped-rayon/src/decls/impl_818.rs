macro_rules! deps {
    () => {
        RevProducer!();
        Rev!();
        Producer!();
        IntoIter!();
    };
}

macro_rules! impl_818 {
    () => {
        deps!();
        impl < P > Producer for RevProducer < P > where P : Producer , { type Item = P :: Item ; type IntoIter = iter :: Rev < P :: IntoIter > ; fn into_iter (self) -> Self :: IntoIter { self . base . into_iter () . rev () } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (self . len - index) ; (RevProducer { base : right , len : index , } , RevProducer { base : left , len : self . len - index , } ,) } }
    };
}

impl_818!();