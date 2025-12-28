macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < T : Clone , A : Allocator + Clone > Clone for Vec < T , A > { # [inline (always)] fn clone (& self) -> Self { let alloc = self . allocator () . clone () ; let mut vec = Vec :: with_capacity_in (self . len () , alloc) ; vec . extend_from_slice (self) ; vec } # [inline (always)] fn clone_from (& mut self , other : & Self) { self . truncate (other . len ()) ; let (init , tail) = other . split_at (self . len ()) ; self . clone_from_slice (init) ; self . extend_from_slice (tail) ; } }
    };
}

impl_161!();