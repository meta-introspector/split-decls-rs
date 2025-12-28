macro_rules! DiagnosticCode {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum DiagnosticCode { RustcHardError (& 'static str) , SyntaxError , RustcLint (& 'static str) , Clippy (& 'static str) , Ra (& 'static str , Severity) , }
    };
}

DiagnosticCode!()