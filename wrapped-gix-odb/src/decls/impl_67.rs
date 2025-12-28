macro_rules! deps {
    () => {
        Store!();
        Handle!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < S > Drop for super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , { fn drop (& mut self) { if let Some (token) = self . token . take () { self . store . remove_handle (token) ; } } }
    };
}

impl_67!();