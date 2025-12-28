macro_rules! deps {
    () => {
        CguReuse!();
        Diagnostic!();
    };
}

macro_rules! IncorrectCguReuseType {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_incorrect_cgu_reuse_type)] pub (crate) struct IncorrectCguReuseType < 'a > { # [primary_span] pub span : Span , pub cgu_user_name : & 'a str , pub actual_reuse : CguReuse , pub expected_reuse : CguReuse , pub at_least : u8 , }
    };
}

IncorrectCguReuseType!()