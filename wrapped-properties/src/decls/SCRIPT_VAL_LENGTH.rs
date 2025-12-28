macro_rules! deps {
    () => {
        Script!();
        ScriptWithExt!();
    };
}

macro_rules! SCRIPT_VAL_LENGTH {
    () => {
        deps!();
        # [doc = " The number of bits at the low-end of a `ScriptWithExt` value used for"] # [doc = " storing the `Script` value (or `extensions` index)."] const SCRIPT_VAL_LENGTH : u16 = 10 ;
    };
}

SCRIPT_VAL_LENGTH!()