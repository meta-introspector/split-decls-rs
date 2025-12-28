macro_rules! deps {
    () => {
        Section!();
        Key!();
        NameParameter!();
        Tree!();
    };
}

macro_rules! impl_708 {
    () => {
        deps!();
        impl Section for NameParameter { fn name (& self) -> & str { "<name>" } fn keys (& self) -> & [& dyn Key] { & [& Self :: ALLOW] } fn parent (& self) -> Option < & dyn Section > { Some (& config :: Tree :: PROTOCOL) } }
    };
}

impl_708!()