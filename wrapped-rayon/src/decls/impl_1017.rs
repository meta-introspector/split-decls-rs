macro_rules! deps {
    () => {
        ZipProducer!();
        Zip!();
        IntoIter!();
        Producer!();
    };
}

macro_rules! impl_1017 {
    () => {
        deps!();
        impl < A : Producer , B : Producer > Producer for ZipProducer < A , B > { type Item = (A :: Item , B :: Item) ; type IntoIter = iter :: Zip < A :: IntoIter , B :: IntoIter > ; fn into_iter (self) -> Self :: IntoIter { self . a . into_iter () . zip (self . b . into_iter ()) } fn min_len (& self) -> usize { Ord :: max (self . a . min_len () , self . b . min_len ()) } fn max_len (& self) -> usize { Ord :: min (self . a . max_len () , self . b . max_len ()) } fn split_at (self , index : usize) -> (Self , Self) { let (a_left , a_right) = self . a . split_at (index) ; let (b_left , b_right) = self . b . split_at (index) ; (ZipProducer { a : a_left , b : b_left , } , ZipProducer { a : a_right , b : b_right , } ,) } }
    };
}

impl_1017!()