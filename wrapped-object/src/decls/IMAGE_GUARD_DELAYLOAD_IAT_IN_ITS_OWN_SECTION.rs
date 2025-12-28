macro_rules! IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION {
    () => {
        # [doc = " Delayload import table in its own .didat section (with nothing else in it) that can be freely reprotected"] pub const IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION : u32 = 0x0000_2000 ;
    };
}

IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION!()