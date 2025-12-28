macro_rules! trace {
    () => {
        macro_rules ! trace { ($ ($ tt : tt) *) => { log ! (log :: trace ! ($ ($ tt) *)) } }
    };
}

trace!()