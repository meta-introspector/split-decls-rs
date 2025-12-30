// Generated macro for HelloWorld (struct)
macro_rules! Depcrate_hello_worldHelloWorld {
() => {
// Module: crate::hello_world
// Provides: {"HelloWorld"}
// Dependencies: {}
# [doc = " A struct containing \"Hello World\" in the requested language."] # [derive (Debug , PartialEq , Clone , Yokeable , ZeroFrom)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (any (feature = "deserialize_json" , feature = "export") , derive (serde :: Serialize))] # [cfg_attr (feature = "export" , derive (databake :: Bake))] # [cfg_attr (feature = "export" , databake (path = icu_provider :: hello_world))] pub struct HelloWorld < 'data > { # [doc = " The translation of \"Hello World\"."] # [cfg_attr (feature = "serde" , serde (borrow))] pub message : Cow < 'data , str > , }
};
}
