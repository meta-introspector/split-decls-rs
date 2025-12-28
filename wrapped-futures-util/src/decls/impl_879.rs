macro_rules! deps {
    () => {
        FuturesUnordered!();
    };
}

macro_rules! impl_879 {
    () => {
        deps!();
        impl < Fut > FuturesUnordered < Fut > { # [doc = " Clears the set, removing all futures."] pub fn clear (& mut self) { * self = Self :: new () ; } }
    };
}

impl_879!();