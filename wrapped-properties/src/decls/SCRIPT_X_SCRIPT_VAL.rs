macro_rules! deps {
    () => {
        Script!();
        ScriptWithExt!();
    };
}

macro_rules! SCRIPT_X_SCRIPT_VAL {
    () => {
        deps!();
        # [doc = " The bit mask necessary to retrieve the `Script` value (or `extensions` index)"] # [doc = " from a `ScriptWithExt` value."] const SCRIPT_X_SCRIPT_VAL : u16 = (1 << SCRIPT_VAL_LENGTH) - 1 ;
    };
}

SCRIPT_X_SCRIPT_VAL!()