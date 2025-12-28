macro_rules! deps {
    () => {
        DequeInner!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        # [doc = " As with the standard library's `VecDeque`, items are added via `push_back`."] impl < T , S : VecStorage < T > + ? Sized > Extend < T > for DequeInner < T , S > { fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { for item in iter { self . push_back (item) . ok () . unwrap () ; } } }
    };
}

impl_39!()