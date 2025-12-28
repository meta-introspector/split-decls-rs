macro_rules! deps {
    () => {
        Style!();
        Effects!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut style = anstyle::Style::new();"] # [doc = " style |= anstyle::Effects::BOLD.into();"] # [doc = " ```"] impl core :: ops :: BitOrAssign < crate :: Effects > for Style { # [inline] fn bitor_assign (& mut self , other : crate :: Effects) { self . effects |= other ; } }
    };
}

impl_55!()