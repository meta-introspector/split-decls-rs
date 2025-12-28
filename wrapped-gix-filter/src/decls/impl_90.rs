macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl Clone for State { fn clone (& self) -> Self { State { running : Default :: default () , context : self . context . clone () , } } }
    };
}

impl_90!()