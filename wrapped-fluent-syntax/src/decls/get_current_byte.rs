macro_rules! get_current_byte {
    () => {
        macro_rules ! get_current_byte { ($ s : expr) => { $ s . source . as_ref () . as_bytes () . get ($ s . ptr) } ; }
    };
}

get_current_byte!();