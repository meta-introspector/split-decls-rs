macro_rules! deps {
    () => {
        Vec!();
        Box!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < T , const N : usize > From < [T ; N] > for Vec < T > { # [inline (always)] fn from (s : [T ; N]) -> Vec < T > { Box :: slice (Box :: new (s)) . into_vec () } }
    };
}

impl_184!();