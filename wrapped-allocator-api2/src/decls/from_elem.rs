macro_rules! deps {
    () => {
        Vec!();
        ExtendElement!();
    };
}

macro_rules! from_elem {
    () => {
        deps!();
        # [inline (always)] # [cfg (not (no_global_oom_handling))] # [doc (hidden)] pub fn from_elem < T : Clone > (elem : T , n : usize) -> Vec < T > { let mut v = Vec :: with_capacity (n) ; v . extend_with (n , ExtendElement (elem)) ; v }
    };
}

from_elem!()