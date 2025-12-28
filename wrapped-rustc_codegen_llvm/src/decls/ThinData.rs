macro_rules! ThinData {
    () => {
        pub struct ThinData (& 'static mut llvm :: ThinLTOData) ;
    };
}

ThinData!()