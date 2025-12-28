macro_rules! deps {
    () => {
        RingElementNTT!();
    };
}

macro_rules! mat_mul_vec_transposed {
    () => {
        deps!();
        pub fn mat_mul_vec_transposed < const K : usize > (mat : & [[RingElementNTT ; K]] , vec : & [RingElementNTT] ,) -> [RingElementNTT ; K] { let mut ret = [RingElementNTT :: zero () ; K] ; for (i , r) in ret . iter_mut () . enumerate () { for j in 0 .. K { let product = mat [j] [i] * vec [j] ; * r += product ; } } ret }
    };
}

mat_mul_vec_transposed!();