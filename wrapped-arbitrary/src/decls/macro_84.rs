macro_rules! deps {
    () => {
        Arbitrary!();
    };
}

macro_rules! macro_84 {
    () => {
        deps!();
        impl_range ! (RangeTo < A >, | r : & RangeTo < A >| r . end . clone () , A , unbounded_range (| b | .. b) , | depth | < A as Arbitrary >:: try_size_hint (depth)) ;
    };
}

macro_84!()