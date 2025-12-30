// Generated macro for NameKind (enum)
macro_rules! Depcrate_contextNameKind {
() => {
// Module: crate::context
// Provides: {"NameKind"}
// Dependencies: {}
# [doc = " The kind of the name we are completing."] # [derive (Debug)] # [allow (dead_code)] pub (crate) enum NameKind { Const , ConstParam , Enum , Function , IdentPat (PatternContext) , MacroDef , MacroRules , # [doc = " Fake node"] Module (ast :: Module) , RecordField , Rename , SelfParam , Static , Struct , Trait , TypeAlias , TypeParam , Union , Variant , }
};
}
