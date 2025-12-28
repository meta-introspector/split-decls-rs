macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
        ExtendElement!();
    };
}

macro_rules! from_elem_in {
    () => {
        deps!();
        # [inline (always)] # [cfg (not (no_global_oom_handling))] # [doc (hidden)] pub fn from_elem_in < T : Clone , A : Allocator > (elem : T , n : usize , alloc : A) -> Vec < T , A > { let mut v = Vec :: with_capacity_in (n , alloc) ; v . extend_with (n , ExtendElement (elem)) ; v }
    };
}

from_elem_in!();