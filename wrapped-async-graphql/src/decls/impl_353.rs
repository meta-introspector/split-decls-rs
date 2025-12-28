macro_rules! deps {
    () => {
        ContextBase!();
        Directive!();
        InputType!();
        ServerResult!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl < 'a > ContextBase < 'a , & 'a Positioned < Directive > > { # [doc (hidden)] pub fn param_value < T : InputType > (& self , name : & str , default : Option < fn () -> T > ,) -> ServerResult < (Pos , T) > { self . get_param_value (& self . item . node . arguments , name , default) } }
    };
}

impl_353!();