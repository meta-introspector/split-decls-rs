macro_rules! deps {
    () => {
        ScriptWithExt!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        impl From < ScriptWithExt > for u32 { fn from (swe : ScriptWithExt) -> Self { swe . 0 as u32 } }
    };
}

impl_383!();