macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T : std :: fmt :: Debug > Drop for Handle < T > { fn drop (& mut self) { if let Some ((_id , Some (tempfile))) = REGISTRY . remove (& self . id) { tempfile . drop_impl () ; } } }
    };
}

impl_22!();