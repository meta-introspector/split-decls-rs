// Generated macro for impl_180 (impl)
macro_rules! Depcrate_astimpl_180 {
() => {
// Module: crate::ast
// Provides: {"impl_180"}
// Dependencies: {}
impl SelfKind { pub fn to_ref_suggestion (& self) -> String { match self { SelfKind :: Region (None , mutbl) => mutbl . ref_prefix_str () . to_string () , SelfKind :: Region (Some (lt) , mutbl) => format ! ("&{lt} {}" , mutbl . prefix_str ()) , SelfKind :: Pinned (None , mutbl) => format ! ("&pin {}" , mutbl . ptr_str ()) , SelfKind :: Pinned (Some (lt) , mutbl) => format ! ("&{lt} pin {}" , mutbl . ptr_str ()) , SelfKind :: Value (_) | SelfKind :: Explicit (_ , _) => { unreachable ! ("if we had an explicit self, we wouldn't be here") } } } }
};
}
