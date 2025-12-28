macro_rules! deps {
    () => {
        Task!();
        Empty!();
    };
}

macro_rules! Dequeue {
    () => {
        deps!();
        pub (super) enum Dequeue < Fut > { Data (* const Task < Fut >) , Empty , Inconsistent , }
    };
}

Dequeue!();