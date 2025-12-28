macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! S_DTRACE_DOF {
    () => {
        deps!();
        # [doc = " section contains DTrace Object Format"] pub const S_DTRACE_DOF : u32 = 0xf ;
    };
}

S_DTRACE_DOF!();