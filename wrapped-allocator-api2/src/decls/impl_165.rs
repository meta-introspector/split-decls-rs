macro_rules! deps {
    () => {
        Vec!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] impl < T > FromIterator < T > for Vec < T > { # [inline (always)] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Vec < T > { let mut vec = Vec :: new () ; vec . extend (iter) ; vec } }
    };
}

impl_165!();