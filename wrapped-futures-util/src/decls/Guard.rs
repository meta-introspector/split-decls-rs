macro_rules! Guard {
    () => {
        struct Guard < 'a > { buf : & 'a mut Vec < u8 > , len : usize , }
    };
}

Guard!();