macro_rules! deps {
    () => {
        Empty!();
        Task!();
    };
}

macro_rules! Dequeue {
    () => {
        deps!();
        pub (super) enum Dequeue < Fut > { Data (* const Task < Fut >) , Empty , Inconsistent , }
    };
}

Dequeue!()