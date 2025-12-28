macro_rules! deps {
    () => {
        IntoUrlSealed!();
    };
}

macro_rules! IntoUrl {
    () => {
        deps!();
        # [doc = " A trait to try to convert some type into a `Url`."] # [doc = ""] # [doc = " This trait is \"sealed\", such that only types within reqwest can"] # [doc = " implement it."] pub trait IntoUrl : IntoUrlSealed { }
    };
}

IntoUrl!()