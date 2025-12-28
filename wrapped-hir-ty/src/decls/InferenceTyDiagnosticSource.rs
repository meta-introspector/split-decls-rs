macro_rules! deps {
    () => {
        Diagnostics!();
    };
}

macro_rules! InferenceTyDiagnosticSource {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub enum InferenceTyDiagnosticSource { # [doc = " Diagnostics that come from types in the body."] Body , # [doc = " Diagnostics that come from types in fn parameters/return type, or static & const types."] Signature , }
    };
}

InferenceTyDiagnosticSource!()