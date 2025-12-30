// Generated macro for Direction (enum)
macro_rules! DepcrateDirection {
() => {
// Module: crate
// Provides: {"Direction"}
// Dependencies: {}
# [doc = " Edge direction."] # [derive (Clone , Copy , Debug , PartialEq , PartialOrd , Ord , Eq , Hash)] # [repr (usize)] # [cfg_attr (feature = "serde-1" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] pub enum Direction { # [doc = " An `Outgoing` edge is an outward edge *from* the current node."] Outgoing = 0 , # [doc = " An `Incoming` edge is an inbound edge *to* the current node."] Incoming = 1 , }
};
}
