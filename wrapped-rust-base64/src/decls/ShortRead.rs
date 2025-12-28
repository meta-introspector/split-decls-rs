macro_rules! ShortRead {
    () => {
        struct ShortRead < R : io :: Read > { delegate : R , max_read_len : usize , }
    };
}

ShortRead!();