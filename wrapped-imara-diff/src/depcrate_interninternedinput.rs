// Generated macro for InternedInput (struct)
macro_rules! Depcrate_internInternedInput {
() => {
// Module: crate::intern
// Provides: {"InternedInput"}
// Dependencies: {}
# [doc = " Two lists of interned [tokens](crate::intern::Token) that a [`Diff`](crate::Diff) can be computed from."] # [doc = ""] # [doc = " A token represents the smallest possible unit of change during a diff."] # [doc = " For text this is usually a line, a word or a single character."] # [doc = " All [algorithms](crate::Algorithm) operate on interned tokens instead"] # [doc = " of using the token data directly."] # [doc = " This allows for much better performance by amortizing the cost of hashing/equality."] # [doc = ""] # [doc = " While you can intern tokens yourself it is strongly recommended to use [`InternedInput`] module."] # [derive (Default)] pub struct InternedInput < T > { pub before : Vec < Token > , pub after : Vec < Token > , pub interner : Interner < T > , }
};
}
