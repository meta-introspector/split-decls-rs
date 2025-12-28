macro_rules! deps {
    () => {
        SmallIndex!();
    };
}

macro_rules! impl_410 {
    () => {
        deps!();
        impl < T > core :: ops :: IndexMut < SmallIndex > for [T] { # [inline] fn index_mut (& mut self , index : SmallIndex) -> & mut T { & mut self [index . as_usize ()] } }
    };
}

impl_410!()