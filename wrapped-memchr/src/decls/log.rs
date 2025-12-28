macro_rules! log {
    () => {
        macro_rules ! log { ($ ($ tt : tt) *) => { # [cfg (feature = "logging")] { $ ($ tt) * } } }
    };
}

log!()