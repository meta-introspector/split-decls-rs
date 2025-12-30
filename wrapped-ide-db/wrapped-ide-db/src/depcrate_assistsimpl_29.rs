// Generated macro for impl_29 (impl)
macro_rules! Depcrate_assistsimpl_29 {
() => {
// Module: crate::assists
// Provides: {"impl_29"}
// Dependencies: {}
impl AssistKind { pub fn contains (self , other : AssistKind) -> bool { if self == other { return true ; } match self { AssistKind :: Generate => true , AssistKind :: Refactor => matches ! (other , AssistKind :: RefactorExtract | AssistKind :: RefactorInline | AssistKind :: RefactorRewrite) , _ => false , } } pub fn name (& self) -> & str { match self { AssistKind :: QuickFix => "QuickFix" , AssistKind :: Generate => "Generate" , AssistKind :: Refactor => "Refactor" , AssistKind :: RefactorExtract => "RefactorExtract" , AssistKind :: RefactorInline => "RefactorInline" , AssistKind :: RefactorRewrite => "RefactorRewrite" , } } }
};
}
