macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        impl core :: ops :: Index < Span > for [u8] { type Output = [u8] ; # [inline] fn index (& self , index : Span) -> & [u8] { & self [index . range ()] } }
    };
}

impl_448!();