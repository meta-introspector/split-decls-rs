macro_rules! deps {
    () => {
        Style!();
        Effects!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " let style: anstyle::Style = anstyle::Effects::BOLD.into();"] # [doc = " ```"] impl From < crate :: Effects > for Style { # [inline] fn from (effects : crate :: Effects) -> Self { Self :: new () . effects (effects) } }
    };
}

impl_53!()