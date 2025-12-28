macro_rules! dbg {
    () => {
        macro_rules ! dbg { ($ ($ t : tt) *) => { $ ($ t) * } ; }
    };
}

dbg!()