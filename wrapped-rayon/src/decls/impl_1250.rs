macro_rules! deps {
    () => {
        Iter!();
        IterProducer!();
        IntoIter!();
        Producer!();
    };
}

macro_rules! impl_1250 {
    () => {
        deps!();
        impl < 'data , T : 'data + Sync > Producer for IterProducer < 'data , T > { type Item = & 'data T ; type IntoIter = :: std :: slice :: Iter < 'data , T > ; fn into_iter (self) -> Self :: IntoIter { self . slice . iter () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . slice . split_at (index) ; (IterProducer { slice : left } , IterProducer { slice : right }) } }
    };
}

impl_1250!()