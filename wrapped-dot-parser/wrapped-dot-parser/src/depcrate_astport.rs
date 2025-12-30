// Generated macro for Port (enum)
macro_rules! Depcrate_astPort {
() => {
// Module: crate::ast
// Provides: {"Port"}
// Dependencies: {}
# [doc = " This enum corresponds to the `port` non-terminal of the grammar."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash , Clone)] pub enum Port { # [doc = " The variant in which an ID is given, and possibly a compass point."] ID (String , Option < CompassPt >) , # [doc = " The variant in which only a compass point is given."] Compass (CompassPt) , }
};
}
