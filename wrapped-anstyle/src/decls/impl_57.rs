macro_rules! deps {
    () => {
        Effects!();
        Style!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut style = anstyle::Style::new().bold().underline();"] # [doc = " style -= anstyle::Effects::BOLD.into();"] # [doc = " ```"] impl core :: ops :: SubAssign < crate :: Effects > for Style { # [inline] fn sub_assign (& mut self , other : crate :: Effects) { self . effects -= other ; } }
    };
}

impl_57!()