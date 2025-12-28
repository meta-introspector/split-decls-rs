macro_rules! get_byte {
    () => {
        macro_rules ! get_byte { ($ s : expr , $ idx : expr) => { $ s . source . as_ref () . as_bytes () . get ($ idx) } ; }
    };
}

get_byte!();