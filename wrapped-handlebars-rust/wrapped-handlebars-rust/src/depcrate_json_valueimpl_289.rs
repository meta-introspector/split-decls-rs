// Generated macro for impl_289 (impl)
macro_rules! Depcrate_json_valueimpl_289 {
() => {
// Module: crate::json::value
// Provides: {"impl_289"}
// Dependencies: {}
impl < 'rc > PathAndJson < 'rc > { pub fn new (relative_path : Option < String > , value : ScopedJson < 'rc >) -> PathAndJson < 'rc > { PathAndJson { relative_path , value , } } # [doc = " Returns relative path when the value is referenced"] # [doc = " If the value is from a literal, the path is `None`"] pub fn relative_path (& self) -> Option < & String > { self . relative_path . as_ref () } # [doc = " Returns full path to this value if any"] pub fn context_path (& self) -> Option < & Vec < String > > { self . value . context_path () } # [doc = " Returns the value"] pub fn value (& self) -> & Json { self . value . as_json () } # [doc = " Returns the value, if it is a constant. Otherwise returns None."] pub fn try_get_constant_value (& self) -> Option < & 'rc Json > { match & self . value { ScopedJson :: Constant (value) => Some (* value) , ScopedJson :: Context (_ , _) | ScopedJson :: Derived (_) | ScopedJson :: Missing => None , } } # [doc = " Test if value is missing"] pub fn is_value_missing (& self) -> bool { self . value . is_missing () } pub fn render (& self) -> String { self . value . render () } }
};
}
