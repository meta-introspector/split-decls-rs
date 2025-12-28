macro_rules! deps {
    () => {
        SmallIndex!();
    };
}

macro_rules! impl_412 {
    () => {
        deps!();
        impl < T > core :: ops :: IndexMut < SmallIndex > for Vec < T > { # [inline] fn index_mut (& mut self , index : SmallIndex) -> & mut T { & mut self [index . as_usize ()] } }
    };
}

impl_412!()