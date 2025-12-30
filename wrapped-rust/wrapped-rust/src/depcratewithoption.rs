// Generated macro for WithOption (enum)
macro_rules! DepcrateWithOption {
() => {
// Module: crate
// Provides: {"WithOption"}
// Dependencies: {}
# [doc = " Options for with \"with\" remappings."] # [derive (Debug , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize) , serde (rename_all = "kebab-case"))] pub enum WithOption { Path (String) , Generate , }
};
}
