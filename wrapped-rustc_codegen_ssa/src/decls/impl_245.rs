macro_rules! deps {
    () => {
        ExtraBackendMethods!();
        Coordinator!();
        CompiledModules!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < B : ExtraBackendMethods > Coordinator < B > { fn join (mut self) -> std :: thread :: Result < Result < CompiledModules , () > > { self . future . take () . unwrap () . join () } }
    };
}

impl_245!();