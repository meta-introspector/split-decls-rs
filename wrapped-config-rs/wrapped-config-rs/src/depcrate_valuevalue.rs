// Generated macro for Value (struct)
macro_rules! Depcrate_valueValue {
() => {
// Module: crate::value
// Provides: {"Value"}
// Dependencies: {}
# [doc = " A configuration value."] # [derive (Default , Debug , Clone , PartialEq)] pub struct Value { # [doc = " A description of the original location of the value."] # [doc = ""] # [doc = " A Value originating from a File might contain:"] # [doc = " ```text"] # [doc = " Settings.toml"] # [doc = " ```"] # [doc = ""] # [doc = " A Value originating from the environment would contain:"] # [doc = " ```text"] # [doc = " the environment"] # [doc = " ```"] # [doc = ""] # [doc = " A Value originating from a remote source might contain:"] # [doc = " ```text"] # [doc = " etcd+http://127.0.0.1:2379"] # [doc = " ```"] origin : Option < String > , # [doc = " Underlying kind of the configuration value."] pub kind : ValueKind , }
};
}
