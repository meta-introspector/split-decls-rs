macro_rules! MH_NOFIXPREBINDING {
    () => {
        # [doc = " do not have dyld notify the prebinding agent about this executable"] pub const MH_NOFIXPREBINDING : u32 = 0x400 ;
    };
}

MH_NOFIXPREBINDING!();