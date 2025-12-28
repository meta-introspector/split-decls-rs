macro_rules! deps {
    () => {
        Effects!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let effects = anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE;"] # [doc = " assert_eq!(format!(\"{:?}\", effects), \"Effects(BOLD | UNDERLINE)\");"] # [doc = " ```"] impl core :: ops :: BitOr for Effects { type Output = Self ; # [inline (always)] fn bitor (self , rhs : Self) -> Self { self . insert (rhs) } }
    };
}

impl_29!()