macro_rules! THE_REGISTRY_SET {
    () => {
        static THE_REGISTRY_SET : Once = Once :: new () ;
    };
}

THE_REGISTRY_SET!()