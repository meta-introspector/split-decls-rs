macro_rules! SEG_IMPORT {
    () => {
        # [doc = " the segment for the self (dyld) modifying code stubs that has read, write and execute permissions"] pub const SEG_IMPORT : & str = "__IMPORT" ;
    };
}

SEG_IMPORT!();