macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        # [cfg (feature = "malloc_size_of")] impl < T : MallocSizeOf , const N : usize > MallocSizeOf for SmallVec < T , N > { fn size_of (& self , ops : & mut MallocSizeOfOps) -> usize { let mut n = self . shallow_size_of (ops) ; for elem in self . iter () { n += elem . size_of (ops) ; } n } }
    };
}

impl_164!()