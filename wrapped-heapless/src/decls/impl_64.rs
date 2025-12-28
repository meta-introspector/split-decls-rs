macro_rules! deps {
    () => {
        HistoryBufInner!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < T , S : HistoryBufStorage < T > + ? Sized > Extend < T > for HistoryBufInner < T , S > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = T > , { for item in iter . into_iter () { self . write (item) ; } } }
    };
}

impl_64!()