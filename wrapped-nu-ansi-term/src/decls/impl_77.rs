macro_rules! deps {
    () => {
        Rgb!();
        ANSIColorCode!();
        TargetGround!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl ANSIColorCode for Rgb { fn ansi_color_code (& self , target : TargetGround) -> String { format ! ("{};2;{};{};{}" , target . code () + 8 , self . r , self . g , self . b) } }
    };
}

impl_77!()