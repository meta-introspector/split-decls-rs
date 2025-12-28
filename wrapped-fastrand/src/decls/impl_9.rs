macro_rules! deps {
    () => {
        RestoreOnDrop!();
        Rng!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Drop for RestoreOnDrop < '_ > { fn drop (& mut self) { self . rng . set (Rng (self . current . 0)) ; } }
    };
}

impl_9!()