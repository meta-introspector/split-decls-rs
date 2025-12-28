macro_rules! deps {
    () => {
        Arbitrary!();
    };
}

macro_rules! macro_83 {
    () => {
        deps!();
        impl_range ! (RangeInclusive < A >, | r : & RangeInclusive < A >| (r . start () . clone () , r . end () . clone ()) , (A , A) , bounded_range (| (a , b) | a ..= b) , | depth | Ok (crate :: size_hint :: and (< A as Arbitrary >:: try_size_hint (depth) ?, < A as Arbitrary >:: try_size_hint (depth) ?,))) ;
    };
}

macro_83!();