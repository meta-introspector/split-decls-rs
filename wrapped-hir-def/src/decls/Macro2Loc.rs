macro_rules! deps {
    () => {
        MacroExpander!();
        ModuleId!();
    };
}

macro_rules! Macro2Loc {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Macro2Loc { pub container : ModuleId , pub id : AstId < ast :: MacroDef > , pub expander : MacroExpander , pub allow_internal_unsafe : bool , pub edition : Edition , }
    };
}

Macro2Loc!()