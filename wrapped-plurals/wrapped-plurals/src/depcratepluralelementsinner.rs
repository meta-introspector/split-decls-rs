// Generated macro for PluralElementsInner (struct)
macro_rules! DepcratePluralElementsInner {
() => {
// Module: crate
// Provides: {"PluralElementsInner"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize))] pub (crate) struct PluralElementsInner < T > { # [cfg_attr (feature = "serde" , serde (skip_serializing_if = "Option::is_none"))] zero : Option < T > , # [cfg_attr (feature = "serde" , serde (skip_serializing_if = "Option::is_none"))] one : Option < T > , # [cfg_attr (feature = "serde" , serde (skip_serializing_if = "Option::is_none"))] two : Option < T > , # [cfg_attr (feature = "serde" , serde (skip_serializing_if = "Option::is_none"))] few : Option < T > , # [cfg_attr (feature = "serde" , serde (skip_serializing_if = "Option::is_none"))] many : Option < T > , other : T , # [cfg_attr (feature = "serde" , serde (skip_serializing_if = "Option::is_none"))] explicit_zero : Option < T > , # [cfg_attr (feature = "serde" , serde (skip_serializing_if = "Option::is_none"))] explicit_one : Option < T > , }
};
}
