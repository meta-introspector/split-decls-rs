macro_rules! deps {
    () => {
        PatternContext!();
    };
}

macro_rules! NameKind {
    () => {
        deps!();
        # [doc = " The kind of the name we are completing."] # [derive (Debug)] # [allow (dead_code)] pub (crate) enum NameKind { Const , ConstParam , Enum , Function , IdentPat (PatternContext) , MacroDef , MacroRules , # [doc = " Fake node"] Module (ast :: Module) , RecordField , Rename , SelfParam , Static , Struct , Trait , TypeAlias , TypeParam , Union , Variant , }
    };
}

NameKind!();