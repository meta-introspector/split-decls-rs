macro_rules! EF_PARISC_NO_KABP {
    () => {
        # [doc = " No kernel assisted branch prediction."] pub const EF_PARISC_NO_KABP : u32 = 0x0010_0000 ;
    };
}

EF_PARISC_NO_KABP!()