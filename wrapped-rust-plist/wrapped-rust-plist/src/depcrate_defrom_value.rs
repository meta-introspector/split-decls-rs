// Generated macro for from_value (function)
macro_rules! Depcrate_defrom_value {
() => {
// Module: crate::de
// Provides: {"from_value"}
// Dependencies: {}
# [doc = " Interprets a [`Value`] as an instance of type `T`."] pub fn from_value < T : de :: DeserializeOwned > (value : & Value) -> Result < T , Error > { let events = value . events () . map (Ok) ; from_stream (events) }
};
}
