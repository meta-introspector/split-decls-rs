// Generated macro for POINTERS (const)
macro_rules! DepcratePOINTERS {
() => {
// Module: crate
// Provides: {"POINTERS"}
// Dependencies: {}
# [doc = " POINTERS format is described by struct Ptr"] const POINTERS : & [Ptr] = unsafe { core :: slice :: from_raw_parts (POINTERS_BYTES . as_ptr () . cast () , POINTERS_BYTES . len () / core :: mem :: size_of :: < Ptr > ()) } ;
};
}
