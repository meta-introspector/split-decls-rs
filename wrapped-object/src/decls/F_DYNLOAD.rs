macro_rules! F_DYNLOAD {
    () => {
        # [doc = " Indicates the file is dynamically loadable and executable. External references"] # [doc = " are resolved by way of imports, and the file might contain exports and loader"] # [doc = " relocation."] pub const F_DYNLOAD : u16 = 0x1000 ;
    };
}

F_DYNLOAD!();