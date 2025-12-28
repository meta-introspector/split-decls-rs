macro_rules! VariableInfo {
    () => {
        # [doc = " Information about a variable found in the AST"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct VariableInfo { pub name : String , pub type_name : String , pub is_mutable : bool , pub scope : String , }
    };
}

VariableInfo!();