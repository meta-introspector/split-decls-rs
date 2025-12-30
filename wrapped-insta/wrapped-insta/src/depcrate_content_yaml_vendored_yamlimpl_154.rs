// Generated macro for impl_154 (impl)
macro_rules! Depcrate_content_yaml_vendored_yamlimpl_154 {
() => {
// Module: crate::content::yaml::vendored::yaml
// Provides: {"impl_154"}
// Dependencies: {}
impl Yaml { define_as ! (as_bool , bool , Boolean) ; define_as ! (as_i64 , i64 , Integer) ; define_as_ref ! (as_str , & str , String) ; define_as_ref ! (as_hash , & Hash , Hash) ; define_as_ref ! (as_vec , & Array , Array) ; define_into ! (into_bool , bool , Boolean) ; define_into ! (into_i64 , i64 , Integer) ; define_into ! (into_string , String , String) ; define_into ! (into_hash , Hash , Hash) ; define_into ! (into_vec , Array , Array) ; pub fn is_null (& self) -> bool { matches ! (* self , Yaml :: Null) } pub fn is_badvalue (& self) -> bool { matches ! (* self , Yaml :: BadValue) } pub fn is_array (& self) -> bool { matches ! (* self , Yaml :: Array (_)) } pub fn as_f64 (& self) -> Option < f64 > { match * self { Yaml :: Real (ref v) => parse_f64 (v) , _ => None , } } pub fn into_f64 (self) -> Option < f64 > { match self { Yaml :: Real (ref v) => parse_f64 (v) , _ => None , } } }
};
}
