macro_rules! deps {
    () => {
        OptimizationDiagnosticKind!();
        OptimizationDiagnostic!();
    };
}

macro_rules! impl_407 {
    () => {
        deps!();
        impl < 'll > OptimizationDiagnostic < 'll > { unsafe fn unpack (kind : OptimizationDiagnosticKind , di : & 'll DiagnosticInfo) -> Self { let mut function = None ; let mut line = 0 ; let mut column = 0 ; let mut message = None ; let mut filename = None ; let pass_name = super :: build_string (| pass_name | { message = super :: build_string (| message | { filename = super :: build_string (| filename | unsafe { super :: LLVMRustUnpackOptimizationDiagnostic (di , pass_name , & mut function , & mut line , & mut column , filename , message ,) }) . ok () }) . ok () }) . ok () ; let mut filename = filename . unwrap_or_default () ; if filename . is_empty () { filename . push_str ("<unknown file>") ; } OptimizationDiagnostic { kind , pass_name : pass_name . expect ("got a non-UTF8 pass name from LLVM") , function : function . unwrap () , line , column , filename , message : message . expect ("got a non-UTF8 OptimizationDiagnostic message from LLVM") , } } }
    };
}

impl_407!()