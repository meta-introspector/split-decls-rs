macro_rules! MacroExpander {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum MacroExpander { Declarative , BuiltIn (BuiltinFnLikeExpander) , BuiltInAttr (BuiltinAttrExpander) , BuiltInDerive (BuiltinDeriveExpander) , BuiltInEager (EagerExpander) , }
    };
}

MacroExpander!();