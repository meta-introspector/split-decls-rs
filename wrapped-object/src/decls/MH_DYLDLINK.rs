macro_rules! MH_DYLDLINK {
    () => {
        # [doc = " the object file is input for the dynamic linker and can't be statically link edited again"] pub const MH_DYLDLINK : u32 = 0x4 ;
    };
}

MH_DYLDLINK!();