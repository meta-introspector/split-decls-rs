macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
        IntoIter!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < T : Clone , A : Allocator + Clone > Clone for IntoIter < T , A > { fn clone (& self) -> Self { let mut vec = Vec :: < T , A > :: with_capacity_in (self . len () , (* self . alloc) . clone ()) ; vec . extend (self . as_slice () . iter () . cloned ()) ; vec . into_iter () } }
    };
}

impl_130!()