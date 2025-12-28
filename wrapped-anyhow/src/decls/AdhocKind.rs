macro_rules! deps {
    () => {
        Adhoc!();
    };
}

macro_rules! AdhocKind {
    () => {
        deps!();
        # [doc (hidden)] pub trait AdhocKind : Sized { # [inline] fn anyhow_kind (& self) -> Adhoc { Adhoc } }
    };
}

AdhocKind!();