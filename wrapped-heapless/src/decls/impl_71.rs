macro_rules! deps {
    () => {
        HistoryBuf!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T , const N : usize > Default for HistoryBuf < T , N > { fn default () -> Self { Self :: new () } }
    };
}

impl_71!()