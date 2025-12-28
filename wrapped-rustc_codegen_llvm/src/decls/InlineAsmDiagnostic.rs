macro_rules! deps {
    () => {
        DiagnosticLevel!();
    };
}

macro_rules! InlineAsmDiagnostic {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct InlineAsmDiagnostic { pub level : super :: DiagnosticLevel , pub cookie : u64 , pub message : String , pub source : Option < (String , Vec < InnerSpan >) > , }
    };
}

InlineAsmDiagnostic!();