macro_rules! deps {
    () => {
        Cycle!();
    };
}

macro_rules! impl_489 {
    () => {
        deps!();
        impl < N > Cycle < N > { # [doc = " Return a node id that participates in the cycle"] pub fn node_id (& self) -> N where N : Copy , { self . 0 } }
    };
}

impl_489!()