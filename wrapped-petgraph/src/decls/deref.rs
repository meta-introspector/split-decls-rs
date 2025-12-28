macro_rules! deref {
    () => {
        macro_rules ! deref { ($ e : expr) => { *$ e } ; }
    };
}

deref!()