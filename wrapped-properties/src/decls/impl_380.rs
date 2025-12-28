macro_rules! deps {
    () => {
        ScriptWithExt!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        # [allow (missing_docs)] # [allow (non_upper_case_globals)] # [doc (hidden)] impl ScriptWithExt { pub const Unknown : ScriptWithExt = ScriptWithExt (0) ; }
    };
}

impl_380!()