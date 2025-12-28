macro_rules! deps {
    () => {
        FilterEdge!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < F , N > FilterEdge < N > for F where F : Fn (N) -> bool , { fn include_edge (& self , n : N) -> bool { (* self) (n) } }
    };
}

impl_143!()