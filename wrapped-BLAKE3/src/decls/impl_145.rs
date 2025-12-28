macro_rules! deps {
    () => {
        Join!();
        SerialJoin!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl Join for SerialJoin { # [inline] fn join < A , B , RA , RB > (oper_a : A , oper_b : B) -> (RA , RB) where A : FnOnce () -> RA + Send , B : FnOnce () -> RB + Send , RA : Send , RB : Send , { (oper_a () , oper_b ()) } }
    };
}

impl_145!();