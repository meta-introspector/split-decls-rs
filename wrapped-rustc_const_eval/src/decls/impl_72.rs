macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl State { # [inline] pub (super) fn contains (& self , local : Local) -> bool { self . qualif . contains (local) } }
    };
}

impl_72!();