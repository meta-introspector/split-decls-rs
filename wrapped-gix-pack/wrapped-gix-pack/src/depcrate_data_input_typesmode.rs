// Generated macro for Mode (enum)
macro_rules! Depcrate_data_input_typesMode {
() => {
// Module: crate::data::input::types
// Provides: {"Mode"}
// Dependencies: {}
# [doc = " Iteration Mode"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Mode { # [doc = " Provide the trailer as read from the pack"] AsIs , # [doc = " Generate an own hash and trigger an error on the last iterated object"] # [doc = " if it does not match the hash provided with the pack."] # [doc = ""] # [doc = " This way the one iterating the data cannot miss corruption as long as"] # [doc = " the iteration is continued through to the end."] Verify , # [doc = " Generate an own hash and if there was an error or the objects are depleted early"] # [doc = " due to partial packs, return the last valid entry and with our own hash thus far."] # [doc = " Note that the existing pack hash, if present, will be ignored."] # [doc = " As we won't know which objects fails, every object will have the hash obtained thus far."] # [doc = " This also means that algorithms must know about this possibility, or else might wrongfully"] # [doc = " assume the pack is finished."] Restore , }
};
}
