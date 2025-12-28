macro_rules! div_rem {
    () => {
        fn div_rem (x : usize , d : usize) -> (usize , usize) { (x / d , x % d) }
    };
}

div_rem!()