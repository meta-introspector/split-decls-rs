macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < T : Clone , const N : usize > Clone for SmallVec < T , N > { # [inline] fn clone (& self) -> SmallVec < T , N > { SmallVec :: from (self . as_slice ()) } # [inline] fn clone_from (& mut self , source : & Self) { # [cfg (feature = "specialization")] { < Self as spec_traits :: SpecCloneFrom < T > > :: spec_clone_from (self , source) ; } # [cfg (not (feature = "specialization"))] { self . clone_from_fallback (& * source) ; } } }
    };
}

impl_132!();