macro_rules! swap_pair {
    () => {
        fn swap_pair < T > (mut x : [T ; 2]) -> [T ; 2] { x . swap (0 , 1) ; x }
    };
}

swap_pair!()