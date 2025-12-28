macro_rules! CALLBACK {
    () => {
        # [doc = " Use an atomic pointer to store the global tracing subscriber function."] static CALLBACK : AtomicPtr < () > = AtomicPtr :: new (std :: ptr :: null_mut ()) ;
    };
}

CALLBACK!();