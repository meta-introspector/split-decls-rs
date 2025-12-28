macro_rules! deps {
    () => {
        ModuleCodegen!();
        WriteBackendMethods!();
        SerializedModule!();
    };
}

macro_rules! FatLtoInput {
    () => {
        deps!();
        pub enum FatLtoInput < B : WriteBackendMethods > { Serialized { name : String , buffer : SerializedModule < B :: ModuleBuffer > } , InMemory (ModuleCodegen < B :: Module >) , }
    };
}

FatLtoInput!()