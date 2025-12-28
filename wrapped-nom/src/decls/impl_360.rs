macro_rules! deps {
    () => {
        ToUsize!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        # [cfg (any (target_pointer_width = "32" , target_pointer_width = "64"))] impl ToUsize for u32 { # [inline] fn to_usize (& self) -> usize { * self as usize } }
    };
}

impl_360!()