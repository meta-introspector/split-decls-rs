macro_rules! warn {
    () => {
        macro_rules ! warn { ($ ($ t : tt) *) => { log ! (warn , $ ($ t) *) } }
    };
}

warn!();