macro_rules! deps {
    () => {
        Effects!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut effects = anstyle::Effects::BOLD;"] # [doc = " effects |= anstyle::Effects::UNDERLINE;"] # [doc = " assert_eq!(format!(\"{:?}\", effects), \"Effects(BOLD | UNDERLINE)\");"] # [doc = " ```"] impl core :: ops :: BitOrAssign for Effects { # [inline] fn bitor_assign (& mut self , other : Self) { * self = self . insert (other) ; } }
    };
}

impl_30!()