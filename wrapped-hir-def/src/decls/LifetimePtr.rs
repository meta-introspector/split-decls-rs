macro_rules! LifetimePtr {
    () => {
        pub type LifetimePtr = AstPtr < ast :: Lifetime > ;
    };
}

LifetimePtr!();