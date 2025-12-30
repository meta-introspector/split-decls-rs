// Generated macro for shared_module (function)
macro_rules! Depcrate_encodeshared_module {
() => {
// Module: crate::encode
// Provides: {"shared_module"}
// Dependencies: {}
fn shared_module < 'a > (m : & 'a ast :: ImportModule , intern : & 'a Interner , linked_module : bool ,) -> Result < ImportModule < 'a > , Diagnostic > { Ok (match m { ast :: ImportModule :: Named (m , span) => { intern . resolve_import_module (m , * span , linked_module) ? } ast :: ImportModule :: RawNamed (m , _span) => ImportModule :: RawNamed (intern . intern_str (m)) , ast :: ImportModule :: Inline (idx) => ImportModule :: Inline (* idx as u32) , }) }
};
}
