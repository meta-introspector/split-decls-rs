macro_rules! debug {
    () => {
        macro_rules ! debug { ($ ($ tt : tt) *) => { log ! (log :: debug ! ($ ($ tt) *)) } }
    };
}

debug!()