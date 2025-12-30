// Generated macro for LoopingLookup (struct)
macro_rules! Depcrate_lookupLoopingLookup {
() => {
// Module: crate::lookup
// Provides: {"LoopingLookup"}
// Dependencies: {}
# [doc = " Some functions (e.g. `find_frames`) require considering multiple"] # [doc = " compilation units, each of which might require their own split DWARF"] # [doc = " lookup (and thus produce a continuation)."] # [doc = ""] # [doc = " We store the underlying continuation here as well as a mutator function"] # [doc = " that will either a) decide that the result of this continuation is"] # [doc = " what is needed and mutate it to the final result or b) produce another"] # [doc = " `LookupResult`. `new_lookup` will in turn eagerly drive any non-continuation"] # [doc = " `LookupResult` with successive invocations of the mutator, until a new"] # [doc = " continuation or a final result is produced. And finally, the impl of"] # [doc = " `LookupContinuation::resume` will call `new_lookup` each time the"] # [doc = " computation is resumed."] pub (crate) struct LoopingLookup < T , L , F > where L : LookupContinuation , F : FnMut (L :: Output) -> ControlFlow < T , LookupResult < L > > , { continuation : L , mutator : F , }
};
}
