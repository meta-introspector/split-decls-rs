// Generated macro for BackingStorage (enum)
macro_rules! Depcrate_serializationBackingStorage {
() => {
// Module: crate::serialization
// Provides: {"BackingStorage"}
// Dependencies: {}
# [doc = " The `BackingStorage` is what the data gets written to. Usually that is a"] # [doc = " file but for testing purposes it can also be an in-memory vec of bytes."] # [derive (Debug)] enum BackingStorage { File (fs :: File) , Memory (Vec < u8 >) , }
};
}
