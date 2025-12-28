macro_rules! deps {
    () => {
        WriteBackendMethods!();
        CodegenContext!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < B : WriteBackendMethods > CodegenContext < B > { pub fn create_dcx (& self) -> DiagCtxt { DiagCtxt :: new (Box :: new (self . diag_emitter . clone ())) } }
    };
}

impl_212!()