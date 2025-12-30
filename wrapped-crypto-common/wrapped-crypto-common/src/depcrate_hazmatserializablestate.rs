// Generated macro for SerializableState (trait)
macro_rules! Depcrate_hazmatSerializableState {
() => {
// Module: crate::hazmat
// Provides: {"SerializableState"}
// Dependencies: {}
# [doc = " Types which can serialize the internal state and be restored from it."] # [doc = ""] # [doc = " # Compatibility"] # [doc = ""] # [doc = " Serialized state can be assumed to be stable across backwards compatible"] # [doc = " versions of an implementation crate, i.e. any `0.x.y` version of a crate"] # [doc = " should be able to decode data serialized with any other `0.x.z` version,"] # [doc = " but it may not be able to correctly decode data serialized with a non-`x`"] # [doc = " version."] # [doc = ""] # [doc = " This guarantee is a subject to issues such as security fixes."] # [doc = ""] # [doc = " # SECURITY WARNING"] # [doc = ""] # [doc = " Serialized state may contain sensitive data."] pub trait SerializableState where Self : Sized , { # [doc = " Size of serialized internal state."] type SerializedStateSize : ArraySize ; # [doc = " Serialize and return internal state."] fn serialize (& self) -> SerializedState < Self > ; # [doc = " Create an object from serialized internal state."] fn deserialize (serialized_state : & SerializedState < Self >) -> Result < Self , DeserializeStateError > ; }
};
}
