// Generated macro for AbiAlign (struct)
macro_rules! DepcrateAbiAlign {
() => {
// Module: crate
// Provides: {"AbiAlign"}
// Dependencies: {}
# [doc = " A pair of alignments, ABI-mandated and preferred."] # [doc = ""] # [doc = " The \"preferred\" alignment is an LLVM concept that is virtually meaningless to Rust code:"] # [doc = " it is not exposed semantically to programmers nor can they meaningfully affect it."] # [doc = " The only concern for us is that preferred alignment must not be less than the mandated alignment"] # [doc = " and thus in practice the two values are almost always identical."] # [doc = ""] # [doc = " An example of a rare thing actually affected by preferred alignment is aligning of statics."] # [doc = " It is of effectively no consequence for layout in structs and on the stack."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub struct AbiAlign { pub abi : Align , }
};
}
