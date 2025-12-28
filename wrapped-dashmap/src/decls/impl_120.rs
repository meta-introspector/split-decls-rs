macro_rules! deps {
    () => {
        DashSet!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < K : Eq + Hash , S : BuildHasher + Clone > Extend < K > for DashSet < K , S > { fn extend < T : IntoIterator < Item = K > > (& mut self , iter : T) { let iter = iter . into_iter () . map (| k | (k , ())) ; self . inner . extend (iter) } }
    };
}

impl_120!();