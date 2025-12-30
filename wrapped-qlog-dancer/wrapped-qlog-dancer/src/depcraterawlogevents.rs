// Generated macro for RawLogEvents (enum)
macro_rules! DepcrateRawLogEvents {
() => {
// Module: crate
// Provides: {"RawLogEvents"}
// Dependencies: {}
pub enum RawLogEvents { QlogJson { events : Vec < qlog :: events :: Event > } , QlogJsonSeq { events : Vec < qlog :: reader :: Event > } , Netlog , }
};
}
