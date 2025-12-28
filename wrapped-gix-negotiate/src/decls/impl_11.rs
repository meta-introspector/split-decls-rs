macro_rules! deps {
    () => {
        Algorithm!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Default for Algorithm { fn default () -> Self { Self { revs : gix_revwalk :: PriorityQueue :: new () , non_common_revs : 0 , } } }
    };
}

impl_11!()