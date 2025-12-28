macro_rules! flag_test {
    () => {
        macro_rules ! flag_test { ($ features : expr , $ flag : expr) => { ($ features as u32 & $ flag as u32) != 0 } ; }
    };
}

flag_test!()