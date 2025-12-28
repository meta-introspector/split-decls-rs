macro_rules! deps {
    () => {
        ChainSeq!();
        IntoIter!();
        ChainProducer!();
        Folder!();
        Producer!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < A , B > Producer for ChainProducer < A , B > where A : Producer , B : Producer < Item = A :: Item > , { type Item = A :: Item ; type IntoIter = ChainSeq < A :: IntoIter , B :: IntoIter > ; fn into_iter (self) -> Self :: IntoIter { ChainSeq :: new (self . a . into_iter () , self . b . into_iter ()) } fn min_len (& self) -> usize { Ord :: max (self . a . min_len () , self . b . min_len ()) } fn max_len (& self) -> usize { Ord :: min (self . a . max_len () , self . b . max_len ()) } fn split_at (self , index : usize) -> (Self , Self) { if index <= self . a_len { let a_rem = self . a_len - index ; let (a_left , a_right) = self . a . split_at (index) ; let (b_left , b_right) = self . b . split_at (0) ; (ChainProducer :: new (index , a_left , b_left) , ChainProducer :: new (a_rem , a_right , b_right) ,) } else { let (a_left , a_right) = self . a . split_at (self . a_len) ; let (b_left , b_right) = self . b . split_at (index - self . a_len) ; (ChainProducer :: new (self . a_len , a_left , b_left) , ChainProducer :: new (0 , a_right , b_right) ,) } } fn fold_with < F > (self , mut folder : F) -> F where F : Folder < A :: Item > , { folder = self . a . fold_with (folder) ; if folder . full () { folder } else { self . b . fold_with (folder) } } }
    };
}

impl_308!()