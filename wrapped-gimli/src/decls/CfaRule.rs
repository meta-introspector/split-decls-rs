macro_rules! deps {
    () => {
        UnwindExpression!();
        ReaderOffset!();
        Register!();
        Expression!();
    };
}

macro_rules! CfaRule {
    () => {
        deps!();
        # [doc = " The canonical frame address (CFA) recovery rules."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum CfaRule < T : ReaderOffset > { # [doc = " The CFA is given offset from the given register's value."] RegisterAndOffset { # [doc = " The register containing the base value."] register : Register , # [doc = " The offset from the register's base value."] offset : i64 , } , # [doc = " The CFA is obtained by evaluating a DWARF expression program."] Expression (UnwindExpression < T >) , }
    };
}

CfaRule!()