macro_rules! SelfParamPtr {
    () => {
        pub type SelfParamPtr = AstPtr < ast :: SelfParam > ;
    };
}

SelfParamPtr!()