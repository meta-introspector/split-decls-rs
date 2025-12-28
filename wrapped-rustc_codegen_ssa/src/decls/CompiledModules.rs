macro_rules! deps {
    () => {
        CompiledModule!();
    };
}

macro_rules! CompiledModules {
    () => {
        deps!();
        struct CompiledModules { modules : Vec < CompiledModule > , allocator_module : Option < CompiledModule > , }
    };
}

CompiledModules!()