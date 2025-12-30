// Generated macro for FromDeserialized (trait)
macro_rules! Depcrate_serde_utilsFromDeserialized {
() => {
// Module: crate::serde_utils
// Provides: {"FromDeserialized"}
// Dependencies: {}
# [doc = " Map from deserialized representation"] pub trait FromDeserialized : Sized { type Input ; fn from_deserialized < E > (input : Self :: Input) -> Result < Self , E > where E : Error ; }
};
}
