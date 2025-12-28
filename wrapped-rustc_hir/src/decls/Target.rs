macro_rules! deps {
    () => {
        Arm!();
        GenericParam!();
        WherePredicate!();
        Crate!();
        GenericParamKind!();
        Impl!();
        MethodKind!();
        PatField!();
        Mod!();
        ExprField!();
        Variant!();
        Closure!();
        Param!();
    };
}

macro_rules! Target {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Debug , Eq , HashStable_Generic)] pub enum Target { ExternCrate , Use , Static , Const , Fn , Closure , Mod , ForeignMod , GlobalAsm , TyAlias , Enum , Variant , Struct , Field , Union , Trait , TraitAlias , Impl { of_trait : bool } , Expression , Statement , Arm , AssocConst , Method (MethodKind) , AssocTy , ForeignFn , ForeignStatic , ForeignTy , GenericParam { kind : GenericParamKind , has_default : bool } , MacroDef , Param , PatField , ExprField , WherePredicate , MacroCall , Crate , Delegation { mac : bool } , }
    };
}

Target!()