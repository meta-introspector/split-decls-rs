macro_rules! ConstVariableOrigin {
    () => {
        # [derive (Copy , Clone , Debug)] pub struct ConstVariableOrigin { pub span : Span , # [doc = " `DefId` of the const parameter this was instantiated for, if any."] # [doc = ""] # [doc = " This should only be used for diagnostics."] pub param_def_id : Option < DefId > , }
    };
}

ConstVariableOrigin!();