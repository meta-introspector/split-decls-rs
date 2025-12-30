// Generated macro for resolve (function)
macro_rules! Depcrate_envresolve {
() => {
// Module: crate::env
// Provides: {"resolve"}
// Dependencies: {}
fn resolve < 'a > (value : & 'a Content , path : & [& str]) -> Option < & 'a Content > { path . iter () . try_fold (value , | node , segment | match node . resolve_inner () { Content :: Map (fields) => fields . iter () . find (| x | x . 0 . as_str () == Some (segment)) . map (| x | & x . 1) , Content :: Struct (_ , fields) | Content :: StructVariant (_ , _ , _ , fields) => { fields . iter () . find (| x | x . 0 == * segment) . map (| x | & x . 1) } _ => None , }) }
};
}
