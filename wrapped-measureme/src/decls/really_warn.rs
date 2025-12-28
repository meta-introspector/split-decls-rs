macro_rules! really_warn {
    () => {
        macro_rules ! really_warn { ($ msg : literal $ ($ rest : tt) *) => { error ! (concat ! ("[WARNING] " , $ msg) $ ($ rest) *) } }
    };
}

really_warn!();