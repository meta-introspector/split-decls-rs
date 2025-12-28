macro_rules! deps {
    () => {
        LintLevelsBuilder!();
        TopDown!();
    };
}

macro_rules! EarlyContext {
    () => {
        deps!();
        # [doc = " Context for lint checking of the AST, after expansion, before lowering to HIR."] pub struct EarlyContext < 'a > { pub builder : LintLevelsBuilder < 'a , crate :: levels :: TopDown > , pub buffered : LintBuffer , }
    };
}

EarlyContext!();