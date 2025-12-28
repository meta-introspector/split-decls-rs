macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl Path < '_ > { pub fn is_global (& self) -> bool { self . segments . first () . is_some_and (| segment | segment . ident . name == kw :: PathRoot) } }
    };
}

impl_116!();