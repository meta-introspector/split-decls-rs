macro_rules! Mask {
    () => {
        # [doc = " The type of our mask."] # [doc = ""] # [doc = " While we don't expose anyway to configure this in the public API, if one"] # [doc = " really needs less memory usage or support for longer needles, then it is"] # [doc = " suggested to copy the code from this module and modify it to fit your"] # [doc = " needs. The code below is written to be correct regardless of whether Mask"] # [doc = " is a u8, u16, u32, u64 or u128."] type Mask = u16 ;
    };
}

Mask!()