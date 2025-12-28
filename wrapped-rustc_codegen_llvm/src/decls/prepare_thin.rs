macro_rules! deps {
    () => {
        ModuleLlvm!();
        ThinBuffer!();
    };
}

macro_rules! prepare_thin {
    () => {
        deps!();
        pub (crate) fn prepare_thin (module : ModuleCodegen < ModuleLlvm >) -> (String , ThinBuffer) { let name = module . name ; let buffer = ThinBuffer :: new (module . module_llvm . llmod () , true) ; (name , buffer) }
    };
}

prepare_thin!()