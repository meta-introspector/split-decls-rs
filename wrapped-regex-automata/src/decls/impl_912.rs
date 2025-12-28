macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_912 {
    () => {
        deps!();
        impl core :: ops :: Index < Span > for str { type Output = str ; # [inline] fn index (& self , index : Span) -> & str { & self [index . range ()] } }
    };
}

impl_912!();