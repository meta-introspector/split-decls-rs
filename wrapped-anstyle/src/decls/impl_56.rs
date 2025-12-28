macro_rules! deps {
    () => {
        Style!();
        Effects!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let style = anstyle::Style::new().bold().underline() - anstyle::Effects::BOLD.into();"] # [doc = " ```"] impl core :: ops :: Sub < crate :: Effects > for Style { type Output = Self ; # [inline] fn sub (mut self , other : crate :: Effects) -> Self { self . effects -= other ; self } }
    };
}

impl_56!();