use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A pair of alignments, ABI-mandated and preferred.
///
/// The "preferred" alignment is an LLVM concept that is virtually meaningless to Rust code:
/// it is not exposed semantically to programmers nor can they meaningfully affect it.
/// The only concern for us is that preferred alignment must not be less than the mandated alignment
/// and thus in practice the two values are almost always identical.
///
/// An example of a rare thing actually affected by preferred alignment is aligning of statics.
/// It is of effectively no consequence for layout in structs and on the stack.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "nightly", derive(HashStable_Generic))]
pub struct AbiAlign {
    pub abi: Align,
}
