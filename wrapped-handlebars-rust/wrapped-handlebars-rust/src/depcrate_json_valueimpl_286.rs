// Generated macro for impl_286 (impl)
macro_rules! Depcrate_json_valueimpl_286 {
() => {
// Module: crate::json::value
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'rc > ScopedJson < 'rc > { # [doc = " get the JSON reference"] pub fn as_json (& self) -> & Json { match self { ScopedJson :: Constant (j) => j , ScopedJson :: Derived (ref j) => j , ScopedJson :: Context (j , _) => j , _ => & DEFAULT_VALUE , } } pub fn render (& self) -> String { self . as_json () . render () } pub fn is_missing (& self) -> bool { matches ! (self , ScopedJson :: Missing) } pub fn into_derived (self) -> ScopedJson < 'rc > { let v = self . as_json () ; ScopedJson :: Derived (v . clone ()) } pub fn context_path (& self) -> Option < & Vec < String > > { match self { ScopedJson :: Context (_ , ref p) => Some (p) , _ => None , } } }
};
}
