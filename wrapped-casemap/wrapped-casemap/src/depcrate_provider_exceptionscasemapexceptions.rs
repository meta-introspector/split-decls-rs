// Generated macro for CaseMapExceptions (struct)
macro_rules! Depcrate_provider_exceptionsCaseMapExceptions {
() => {
// Module: crate::provider::exceptions
// Provides: {"CaseMapExceptions"}
// Dependencies: {}
# [doc = " This represents case mapping exceptions that can't be represented as a delta applied to"] # [doc = " the original code point. The codepoint"] # [doc = " trie in CaseMapper stores indices into this VarZeroVec."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_casemap :: provider :: exceptions))] # [derive (Debug , Eq , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] pub struct CaseMapExceptions < 'data > { # [cfg_attr (feature = "serde" , serde (borrow))] # [doc = " The list of exceptions"] pub exceptions : VarZeroVec < 'data , ExceptionULE > , }
};
}
