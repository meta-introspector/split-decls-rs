macro_rules! div {
    () => {
        # [cfg (all (not (debug_assertions) , intrinsics_enabled))] macro_rules ! div { ($ a : expr , $ b : expr) => { unsafe { core :: intrinsics :: unchecked_div ($ a , $ b) } } ; }
    };
}

div!();