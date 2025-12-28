macro_rules! deps {
    () => {
        Effects!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;"] # [doc = " effects -= anstyle::Effects::BOLD;"] # [doc = " assert_eq!(format!(\"{:?}\", effects), \"Effects(UNDERLINE)\");"] # [doc = " ```"] impl core :: ops :: SubAssign for Effects { # [inline] fn sub_assign (& mut self , other : Self) { * self = self . remove (other) ; } }
    };
}

impl_32!()