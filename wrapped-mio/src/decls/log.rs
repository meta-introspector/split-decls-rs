macro_rules! log {
    () => {
        macro_rules ! log { ($ level : ident , $ ($ t : tt) *) => { # [cfg (feature = "log")] { log ::$ level ! ($ ($ t) *) } # [cfg (not (feature = "log"))] { if false { let _ = ($ ($ t) *) ; } } } }
    };
}

log!()