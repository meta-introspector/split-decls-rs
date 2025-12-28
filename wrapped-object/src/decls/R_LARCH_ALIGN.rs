macro_rules! R_LARCH_ALIGN {
    () => {
        # [doc = " Delete some bytes to ensure the instruction at PC + A aligned to"] # [doc = " `A.next_power_of_two()`-byte boundary"] pub const R_LARCH_ALIGN : u32 = 102 ;
    };
}

R_LARCH_ALIGN!()