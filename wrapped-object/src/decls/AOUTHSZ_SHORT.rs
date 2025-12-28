macro_rules! AOUTHSZ_SHORT {
    () => {
        # [doc = " Some AIX programs generate auxiliary headers for 32-bit object files that"] # [doc = " end after the data_start field."] pub const AOUTHSZ_SHORT : u16 = 28 ;
    };
}

AOUTHSZ_SHORT!();