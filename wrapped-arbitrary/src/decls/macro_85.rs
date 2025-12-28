macro_rules! deps {
    () => {
        Arbitrary!();
    };
}

macro_rules! macro_85 {
    () => {
        deps!();
        impl_range ! (RangeToInclusive < A >, | r : & RangeToInclusive < A >| r . end . clone () , A , unbounded_range (| b | ..= b) , | depth | < A as Arbitrary >:: try_size_hint (depth)) ;
    };
}

macro_85!();