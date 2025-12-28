macro_rules! deps {
    () => {
        Bucket!();
        RawIterHash!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < T > Iterator for RawIterHash < T > { type Item = Bucket < T > ; fn next (& mut self) -> Option < Bucket < T > > { unsafe { match self . inner . next () { Some (index) => { debug_assert ! (index <= self . inner . bucket_mask) ; let bucket = Bucket :: from_base_index (self . inner . ctrl . cast () , index) ; Some (bucket) } None => None , } } } }
    };
}

impl_111!();