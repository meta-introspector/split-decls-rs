macro_rules! deps {
    () => {
        SerializedModule!();
        WriteBackendMethods!();
    };
}

macro_rules! ThinShared {
    () => {
        deps!();
        pub struct ThinShared < B : WriteBackendMethods > { pub data : B :: ThinData , pub thin_buffers : Vec < B :: ThinBuffer > , pub serialized_modules : Vec < SerializedModule < B :: ModuleBuffer > > , pub module_names : Vec < CString > , }
    };
}

ThinShared!();