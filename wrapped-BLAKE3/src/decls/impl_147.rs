macro_rules! deps {
    () => {
        Join!();
        RayonJoin!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        # [cfg (feature = "rayon")] impl Join for RayonJoin { # [inline] fn join < A , B , RA , RB > (oper_a : A , oper_b : B) -> (RA , RB) where A : FnOnce () -> RA + Send , B : FnOnce () -> RB + Send , RA : Send , RB : Send , { rayon_core :: join (oper_a , oper_b) } }
    };
}

impl_147!();