macro_rules! HirWrite {
    () => {
        pub trait HirWrite : fmt :: Write { fn start_location_link (& mut self , _location : ModuleDefId) { } fn end_location_link (& mut self) { } }
    };
}

HirWrite!();