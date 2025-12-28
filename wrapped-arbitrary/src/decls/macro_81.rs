macro_rules! deps {
    () => {
        Arbitrary!();
    };
}

macro_rules! macro_81 {
    () => {
        deps!();
        impl_range ! (Range < A >, | r : & Range < A >| (r . start . clone () , r . end . clone ()) , (A , A) , bounded_range (| (a , b) | a .. b) , | depth | Ok (crate :: size_hint :: and (< A as Arbitrary >:: try_size_hint (depth) ?, < A as Arbitrary >:: try_size_hint (depth) ?,))) ;
    };
}

macro_81!();