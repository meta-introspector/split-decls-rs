macro_rules! ThinBuffer {
    () => {
        pub struct ThinBuffer (& 'static mut llvm :: ThinLTOBuffer) ;
    };
}

ThinBuffer!()