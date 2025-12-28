macro_rules! deps {
    () => {
        Result!();
        CustomValidator!();
        InputType!();
        InputValueError!();
    };
}

macro_rules! impl_1001 {
    () => {
        deps!();
        impl < T , F , E > CustomValidator < T > for F where T : InputType , E : Into < InputValueError < T > > , F : Fn (& T) -> Result < () , E > , { # [inline] fn check (& self , value : & T) -> Result < () , InputValueError < T > > { (self) (value) . map_err (Into :: into) } }
    };
}

impl_1001!();