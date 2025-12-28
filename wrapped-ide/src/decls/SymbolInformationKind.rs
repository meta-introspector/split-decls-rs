macro_rules! SymbolInformationKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum SymbolInformationKind { AssociatedType , Attribute , Constant , Enum , EnumMember , Field , Function , Macro , Method , Module , Parameter , SelfParameter , StaticMethod , StaticVariable , Struct , Trait , TraitMethod , Type , TypeAlias , TypeParameter , Union , Variable , }
    };
}

SymbolInformationKind!()