macro_rules! deps {
    () => {
        StateID!();
        SparseSetIter!();
    };
}

macro_rules! impl_946 {
    () => {
        deps!();
        impl < 'a > Iterator for SparseSetIter < 'a > { type Item = StateID ; # [cfg_attr (feature = "perf-inline" , inline (always))] fn next (& mut self) -> Option < StateID > { self . 0 . next () . copied () } }
    };
}

impl_946!()