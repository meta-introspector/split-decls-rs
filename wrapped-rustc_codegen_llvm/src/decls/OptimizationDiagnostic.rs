macro_rules! deps {
    () => {
        OptimizationDiagnosticKind!();
    };
}

macro_rules! OptimizationDiagnostic {
    () => {
        deps!();
        pub (crate) struct OptimizationDiagnostic < 'll > { pub kind : OptimizationDiagnosticKind , pub pass_name : String , # [expect (dead_code)] pub function : & 'll Value , pub line : c_uint , pub column : c_uint , pub filename : String , pub message : String , }
    };
}

OptimizationDiagnostic!();