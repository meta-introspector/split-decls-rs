macro_rules! deps {
    () => {
        Producer!();
        IterMut!();
        IntoIter!();
        IterMutProducer!();
    };
}

macro_rules! impl_1261 {
    () => {
        deps!();
        impl < 'data , T : 'data + Send > Producer for IterMutProducer < 'data , T > { type Item = & 'data mut T ; type IntoIter = :: std :: slice :: IterMut < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . iter_mut () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . slice . split_at_mut (index) ; (IterMutProducer { slice : left } , IterMutProducer { slice : right } ,) } }
    };
}

impl_1261!();