macro_rules! DT_TEXTREL {
    () => {
        # [doc = " Reloc might modify .text"] pub const DT_TEXTREL : u32 = 22 ;
    };
}

DT_TEXTREL!()