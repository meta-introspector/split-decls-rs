macro_rules! info {
    () => {
        # [doc = " Print a debug message to stdout. Format is the same as println! or format!"] macro_rules ! info { ($ ($ arg : tt) *) => (if $ crate :: debug_enabled () { println ! ("Criterion.rs DEBUG: {}" , & format ! ($ ($ arg) *)) }) }
    };
}

info!();