macro_rules! ModuleBuffer {
    () => {
        pub struct ModuleBuffer (& 'static mut llvm :: ModuleBuffer) ;
    };
}

ModuleBuffer!();