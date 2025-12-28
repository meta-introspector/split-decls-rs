macro_rules! MacroId {
    () => {
        # [doc = " A macro"] # [derive (Debug , PartialOrd , Ord , Clone , Copy , PartialEq , Eq , Hash , salsa_macros :: Supertype)] pub enum MacroId { Macro2Id (Macro2Id) , MacroRulesId (MacroRulesId) , ProcMacroId (ProcMacroId) , }
    };
}

MacroId!()