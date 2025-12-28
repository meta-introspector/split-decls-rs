macro_rules! SHT_CREL {
    () => {
        # [doc = " Experimental CREL relocations. LLVM will change the value and"] # [doc = " break compatibility in the future."] pub const SHT_CREL : u32 = 0x40000014 ;
    };
}

SHT_CREL!()