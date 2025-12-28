macro_rules! deps {
    () => {
        ScriptWithExt!();
        Script!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl From < ScriptWithExt > for Script { fn from (swe : ScriptWithExt) -> Self { Script (swe . 0) } }
    };
}

impl_384!();