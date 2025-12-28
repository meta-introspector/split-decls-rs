macro_rules! deps {
    () => {
        Producer!();
        IntoIter!();
        RepeatNProducer!();
        Empty!();
    };
}

macro_rules! impl_808 {
    () => {
        deps!();
        impl < T : Clone + Send > Producer for RepeatNProducer < T > { type Item = T ; type IntoIter = Self ; fn into_iter (self) -> Self :: IntoIter { self } fn split_at (self , index : usize) -> (Self , Self) { if let Self :: Repeats (element , count) = self { assert ! (index <= count . get ()) ; match (NonZeroUsize :: new (index) , NonZeroUsize :: new (count . get () - index) ,) { (Some (left) , Some (right)) => (Self :: Repeats (element . clone () , left) , Self :: Repeats (element , right) ,) , (Some (left) , None) => (Self :: Repeats (element , left) , Self :: Empty) , (None , Some (right)) => (Self :: Empty , Self :: Repeats (element , right)) , (None , None) => unreachable ! () , } } else { assert ! (index == 0) ; (Self :: Empty , Self :: Empty) } } }
    };
}

impl_808!()