macro_rules! deps {
    () => {
        Limb!();
        Constants!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl Constants for Limb { const MAX : Self = Self :: MAX ; }
    };
}

impl_157!();