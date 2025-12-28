macro_rules! F_SHROBJ {
    () => {
        # [doc = " Indicates the file is a shared object (shared library). The file is separately"] # [doc = " loadable. That is, it is not normally bound with other objects, and its loader"] # [doc = " exports symbols are used as automatic import symbols for other object files."] pub const F_SHROBJ : u16 = 0x2000 ;
    };
}

F_SHROBJ!()