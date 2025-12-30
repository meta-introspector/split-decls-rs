// Generated macro for Side (enum)
macro_rules! DepcrateSide {
() => {
// Module: crate
// Provides: {"Side"}
// Dependencies: {}
# [doc = " Whether an endpoint was the initiator of a connection"] # [cfg_attr (feature = "arbitrary" , derive (Arbitrary))] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum Side { # [doc = " The initiator of a connection"] Client = 0 , # [doc = " The acceptor of a connection"] Server = 1 , }
};
}
