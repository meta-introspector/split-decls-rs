macro_rules! trace {
    () => {
        macro_rules ! trace { ($ ($ t : tt) *) => { log ! (trace , $ ($ t) *) } }
    };
}

trace!()