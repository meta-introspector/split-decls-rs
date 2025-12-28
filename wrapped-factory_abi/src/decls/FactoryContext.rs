macro_rules! FactoryContext {
    () => {
        pub type FactoryContext = * mut c_void ;
    };
}

FactoryContext!()