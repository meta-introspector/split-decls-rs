macro_rules! int_pat {
    () => {
        macro_rules ! int_pat { () => { sym :: i8 | sym :: u8 | sym :: i16 | sym :: u16 | sym :: i32 | sym :: u32 | sym :: i64 | sym :: u64 | sym :: i128 | sym :: u128 | sym :: isize | sym :: usize } ; }
    };
}

int_pat!();