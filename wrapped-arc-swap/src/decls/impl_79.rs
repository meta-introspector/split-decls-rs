macro_rules! deps {
    () => {
        LocalNode!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl Drop for LocalNode { fn drop (& mut self) { if let Some (node) = self . node . get () { node . start_cooldown () ; } } }
    };
}

impl_79!();