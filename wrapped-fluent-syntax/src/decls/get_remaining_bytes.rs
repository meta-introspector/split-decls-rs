macro_rules! get_remaining_bytes {
    () => {
        macro_rules ! get_remaining_bytes { ($ s : expr) => { $ s . source . as_ref () . as_bytes () . get ($ s . ptr ..) } ; }
    };
}

get_remaining_bytes!();