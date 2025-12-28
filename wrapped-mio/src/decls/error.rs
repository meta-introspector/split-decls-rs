macro_rules! error {
    () => {
        macro_rules ! error { ($ ($ t : tt) *) => { log ! (error , $ ($ t) *) } }
    };
}

error!();