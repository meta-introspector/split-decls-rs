macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! SymbolKind {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub enum SymbolKind { Attribute , BuiltinAttr , Const , ConstParam , Derive , DeriveHelper , Enum , Field , Function , Method , Impl , InlineAsmRegOrRegClass , Label , LifetimeParam , Local , Macro , ProcMacro , Module , SelfParam , SelfType , Static , Struct , ToolModule , Trait , TypeAlias , TypeParam , Union , ValueParam , Variant , }
    };
}

SymbolKind!();