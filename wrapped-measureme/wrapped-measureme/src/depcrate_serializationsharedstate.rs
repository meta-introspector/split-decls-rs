// Generated macro for SharedState (struct)
macro_rules! Depcrate_serializationSharedState {
() => {
// Module: crate::serialization
// Provides: {"SharedState"}
// Dependencies: {}
# [doc = " This state is shared between all `SerializationSink`s writing to the same"] # [doc = " backing storage (e.g. the same file)."] # [derive (Clone , Debug)] struct SharedState (Arc < Mutex < BackingStorage > >) ;
};
}
