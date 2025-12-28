macro_rules! deps {
    () => {
        Effects!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let effects = (anstyle::Effects::BOLD | anstyle::Effects::UNDERLINE) - anstyle::Effects::BOLD;"] # [doc = " assert_eq!(format!(\"{:?}\", effects), \"Effects(UNDERLINE)\");"] # [doc = " ```"] impl core :: ops :: Sub for Effects { type Output = Self ; # [inline] fn sub (self , other : Self) -> Self { self . remove (other) } }
    };
}

impl_31!();