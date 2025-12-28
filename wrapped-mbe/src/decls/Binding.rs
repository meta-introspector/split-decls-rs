macro_rules! deps {
    () => {
        Fragment!();
        MetaVarKind!();
    };
}

macro_rules! Binding {
    () => {
        deps!();
        # [derive (Debug , Clone)] enum Binding < 'a > { Fragment (Fragment < 'a >) , Nested (Vec < Binding < 'a > >) , Empty , Missing (MetaVarKind) , }
    };
}

Binding!()