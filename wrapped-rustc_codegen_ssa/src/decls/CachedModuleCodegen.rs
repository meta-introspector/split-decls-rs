macro_rules! CachedModuleCodegen {
    () => {
        pub (crate) struct CachedModuleCodegen { pub name : String , pub source : WorkProduct , }
    };
}

CachedModuleCodegen!()