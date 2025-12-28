macro_rules! deps {
    () => {
        Cell!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < T : Copy > Clone for Cell < T > { # [track_caller] fn clone (& self) -> Cell < T > { Cell :: new (self . get ()) } }
    };
}

impl_204!();