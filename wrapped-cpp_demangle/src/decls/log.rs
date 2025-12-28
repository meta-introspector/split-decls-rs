macro_rules! log {
    () => {
        # [cfg (not (feature = "logging"))] macro_rules ! log { ($ fmt : expr) => { } ; ($ fmt : expr , $ ($ x : tt) *) => { if false { let _ = format ! ($ fmt , $ ($ x) *) ; } } ; }
    };
}

log!()