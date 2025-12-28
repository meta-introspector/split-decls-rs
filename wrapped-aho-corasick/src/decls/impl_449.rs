macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        impl core :: ops :: IndexMut < Span > for [u8] { # [inline] fn index_mut (& mut self , index : Span) -> & mut [u8] { & mut self [index . range ()] } }
    };
}

impl_449!()