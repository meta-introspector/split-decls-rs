// Generated macro for BusType (enum)
macro_rules! Depcrate_channelBusType {
() => {
// Module: crate::channel
// Provides: {"BusType"}
// Dependencies: {}
# [doc = " Which bus to connect to"] # [derive (Debug , Clone , Copy , PartialOrd , Ord , PartialEq , Eq , Hash)] pub enum BusType { # [doc = " The Session bus - local to every logged in session"] Session = ffi :: DBusBusType :: Session as isize , # [doc = " The system wide bus"] System = ffi :: DBusBusType :: System as isize , # [doc = " The bus that started us, if any"] Starter = ffi :: DBusBusType :: Starter as isize , }
};
}
