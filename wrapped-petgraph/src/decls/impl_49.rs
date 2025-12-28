macro_rules! deps {
    () => {
        Bfs!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < N , VM > Default for Bfs < N , VM > where VM : Default , { fn default () -> Self { Bfs { stack : VecDeque :: new () , discovered : VM :: default () , } } }
    };
}

impl_49!()