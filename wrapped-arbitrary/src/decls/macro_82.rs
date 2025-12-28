macro_rules! deps {
    () => {
        Arbitrary!();
    };
}

macro_rules! macro_82 {
    () => {
        deps!();
        impl_range ! (RangeFrom < A >, | r : & RangeFrom < A >| r . start . clone () , A , unbounded_range (| a | a ..) , | depth | < A as Arbitrary >:: try_size_hint (depth)) ;
    };
}

macro_82!();