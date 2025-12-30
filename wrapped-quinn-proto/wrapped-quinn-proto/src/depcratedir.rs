// Generated macro for Dir (enum)
macro_rules! DepcrateDir {
() => {
// Module: crate
// Provides: {"Dir"}
// Dependencies: {}
# [doc = " Whether a stream communicates data in both directions or only from the initiator"] # [cfg_attr (feature = "arbitrary" , derive (Arbitrary))] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum Dir { # [doc = " Data flows in both directions"] Bi = 0 , # [doc = " Data flows only from the stream's initiator"] Uni = 1 , }
};
}
