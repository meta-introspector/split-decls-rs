macro_rules! deps {
    () => {
        ToUsize!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] impl ToUsize for u64 { # [inline] fn to_usize (& self) -> usize { * self as usize } }
    };
}

impl_361!()