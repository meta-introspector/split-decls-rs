macro_rules! deps {
    () => {
        TargetGround!();
    };
}

macro_rules! ANSIColorCode {
    () => {
        deps!();
        pub trait ANSIColorCode { fn ansi_color_code (& self , target : TargetGround) -> String ; }
    };
}

ANSIColorCode!()