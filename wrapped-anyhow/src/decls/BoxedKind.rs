macro_rules! deps {
    () => {
        Boxed!();
    };
}

macro_rules! BoxedKind {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] # [doc (hidden)] pub trait BoxedKind : Sized { # [inline] fn anyhow_kind (& self) -> Boxed { Boxed } }
    };
}

BoxedKind!()