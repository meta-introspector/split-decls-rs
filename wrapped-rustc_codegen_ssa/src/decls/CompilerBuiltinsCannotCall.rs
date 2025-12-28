macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! CompilerBuiltinsCannotCall {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_compiler_builtins_cannot_call)] pub struct CompilerBuiltinsCannotCall { pub caller : String , pub callee : String , # [primary_span] pub span : Span , }
    };
}

CompilerBuiltinsCannotCall!();