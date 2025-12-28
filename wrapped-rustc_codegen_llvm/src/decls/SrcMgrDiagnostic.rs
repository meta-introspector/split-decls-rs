macro_rules! deps {
    () => {
        DiagnosticLevel!();
    };
}

macro_rules! SrcMgrDiagnostic {
    () => {
        deps!();
        pub (crate) struct SrcMgrDiagnostic { pub level : super :: DiagnosticLevel , pub message : String , pub source : Option < (String , Vec < InnerSpan >) > , }
    };
}

SrcMgrDiagnostic!();