macro_rules! deps {
    () => {
        FilterNode!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < F , N > FilterNode < N > for F where F : Fn (N) -> bool , { fn include_node (& self , n : N) -> bool { (* self) (n) } }
    };
}

impl_112!()