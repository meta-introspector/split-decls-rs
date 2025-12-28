macro_rules! DiagnosticCode {
    () => {
        # [doc = " The error code associated to this diagnostic."] # [derive (Debug , Clone , Serialize , Deserialize , PartialEq , Eq , Hash)] # [cfg_attr (feature = "builder" , derive (Builder))] # [non_exhaustive] # [cfg_attr (feature = "builder" , builder (pattern = "owned" , setter (into)))] pub struct DiagnosticCode { # [doc = " The code itself."] pub code : String , # [doc = " An explanation for the code"] pub explanation : Option < String > , }
    };
}

DiagnosticCode!();