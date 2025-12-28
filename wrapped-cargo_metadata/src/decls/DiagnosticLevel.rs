macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! DiagnosticLevel {
    () => {
        deps!();
        # [doc = " The diagnostic level"] # [derive (Debug , Clone , Copy , Serialize , Deserialize , PartialEq , Eq , Hash)] # [non_exhaustive] # [serde (rename_all = "lowercase")] pub enum DiagnosticLevel { # [doc = " Internal compiler error"] # [serde (rename = "error: internal compiler error")] Ice , # [doc = " Error"] Error , # [doc = " Warning"] Warning , # [doc = " Failure note"] # [serde (rename = "failure-note")] FailureNote , # [doc = " Note"] Note , # [doc = " Help"] Help , }
    };
}

DiagnosticLevel!()