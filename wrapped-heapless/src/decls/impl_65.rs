macro_rules! deps {
    () => {
        HistoryBufInner!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < 'a , T , S : HistoryBufStorage < T > + ? Sized > Extend < & 'a T > for HistoryBufInner < T , S > where T : 'a + Clone , { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = & 'a T > , { self . extend (iter . into_iter () . cloned ()) ; } }
    };
}

impl_65!()