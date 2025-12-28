macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < 'a , T : Clone + 'a , const N : usize > Extend < & 'a T > for SmallVec < T , N > { # [inline] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { # [cfg (feature = "specialization")] { spec_traits :: SpecExtend :: < & 'a T , _ > :: spec_extend (self , iter . into_iter ()) ; } # [cfg (not (feature = "specialization"))] { self . extend_fallback (iter . into_iter () . cloned ()) ; } } }
    };
}

impl_135!()