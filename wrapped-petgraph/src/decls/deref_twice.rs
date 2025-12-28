macro_rules! deref_twice {
    () => {
        macro_rules ! deref_twice { ($ e : expr) => { **$ e } ; }
    };
}

deref_twice!()