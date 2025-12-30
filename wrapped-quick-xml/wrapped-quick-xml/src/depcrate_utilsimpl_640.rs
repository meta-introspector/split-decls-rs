// Generated macro for impl_640 (impl)
macro_rules! Depcrate_utilsimpl_640 {
() => {
// Module: crate::utils
// Provides: {"impl_640"}
// Dependencies: {}
impl < 'i , 's > CowRef < 'i , 's , str > { # [doc = " Supply to the visitor a borrowed string, a string slice, or an owned"] # [doc = " string depending on the kind of input. Unlike [`Self::deserialize_all`],"] # [doc = " only part of [`Self::Owned`] string will be passed to the visitor."] # [doc = ""] # [doc = " Calls"] # [doc = " - `visitor.visit_borrowed_str` if data borrowed from the input"] # [doc = " - `visitor.visit_str` if data borrowed from another source"] # [doc = " - `visitor.visit_string` if data owned by this type"] # [cfg (feature = "serialize")] pub fn deserialize_str < V , E > (self , visitor : V) -> Result < V :: Value , E > where V : Visitor < 'i > , E : Error , { match self { Self :: Input (s) => visitor . visit_borrowed_str (s) , Self :: Slice (s) => visitor . visit_str (s) , Self :: Owned (s) => visitor . visit_string (s) , } } # [doc = " Calls [`Visitor::visit_bool`] with `true` or `false` if text contains"] # [doc = " [valid] boolean representation, otherwise calls [`Self::deserialize_str`]."] # [doc = ""] # [doc = " The valid boolean representations are only `\"true\"`, `\"false\"`, `\"1\"`, and `\"0\"`."] # [doc = ""] # [doc = " [valid]: https://www.w3.org/TR/xmlschema11-2/#boolean"] # [cfg (feature = "serialize")] pub fn deserialize_bool < V , E > (self , visitor : V) -> Result < V :: Value , E > where V : Visitor < 'i > , E : Error , { match self . as_ref () { "1" | "true" => visitor . visit_bool (true) , "0" | "false" => visitor . visit_bool (false) , _ => self . deserialize_str (visitor) , } } }
};
}
