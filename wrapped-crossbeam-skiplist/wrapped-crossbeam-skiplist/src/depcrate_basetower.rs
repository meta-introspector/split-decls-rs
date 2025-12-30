// Generated macro for Tower (struct)
macro_rules! Depcrate_baseTower {
() => {
// Module: crate::base
// Provides: {"Tower"}
// Dependencies: {}
# [doc = " The tower of atomic pointers."] # [doc = ""] # [doc = " The actual size of the tower will vary depending on the height that a node"] # [doc = " was allocated with."] # [repr (C)] struct Tower < K , V > { pointers : [Atomic < Node < K , V > > ; 0] , }
};
}
