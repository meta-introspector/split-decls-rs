macro_rules! access0 {
    () => {
        macro_rules ! access0 { ($ e : expr) => { $ e . 0 } ; }
    };
}

access0!();