macro_rules! deps {
    () => {
        Effects!();
        Style!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let style = anstyle::Style::new() | anstyle::Effects::BOLD.into();"] # [doc = " ```"] impl core :: ops :: BitOr < crate :: Effects > for Style { type Output = Self ; # [inline (always)] fn bitor (mut self , rhs : crate :: Effects) -> Self { self . effects |= rhs ; self } }
    };
}

impl_54!()