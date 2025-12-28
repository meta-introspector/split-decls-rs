macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        # [cfg (feature = "malloc_size_of")] impl < T , const N : usize > MallocShallowSizeOf for SmallVec < T , N > { fn shallow_size_of (& self , ops : & mut MallocSizeOfOps) -> usize { if self . spilled () { unsafe { ops . malloc_size_of (self . as_ptr ()) } } else { 0 } } }
    };
}

impl_163!();