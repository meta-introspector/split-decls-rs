// Generated macro for RENAMES (static)
macro_rules! DepcrateRENAMES {
() => {
// Module: crate
// Provides: {"RENAMES"}
// Dependencies: {}
# [doc = " List of lints which have been renamed."] # [doc = ""] # [doc = " These will get redirects in the output to the new name. The"] # [doc = " format is `(level, [(old_name, new_name), ...])`."] # [doc = ""] # [doc = " Note: This hard-coded list is a temporary hack. The intent is in the"] # [doc = " future to have `rustc` expose this information in some way (like a `-Z`"] # [doc = " flag spitting out JSON). Also, this does not yet support changing the"] # [doc = " level of the lint, which will be more difficult to support, since rustc"] # [doc = " currently does not track that historical information."] static RENAMES : & [(Level , & [(& str , & str)])] = & [(Level :: Allow , & [("single-use-lifetime" , "single-use-lifetimes") , ("elided-lifetime-in-path" , "elided-lifetimes-in-paths") , ("async-idents" , "keyword-idents") , ("disjoint-capture-migration" , "rust-2021-incompatible-closure-captures") , ("keyword-idents" , "keyword-idents-2018") , ("or-patterns-back-compat" , "rust-2021-incompatible-or-patterns") ,] ,) , (Level :: Warn , & [("bare-trait-object" , "bare-trait-objects") , ("unstable-name-collision" , "unstable-name-collisions") , ("unused-doc-comment" , "unused-doc-comments") , ("redundant-semicolon" , "redundant-semicolons") , ("overlapping-patterns" , "overlapping-range-endpoints") , ("non-fmt-panic" , "non-fmt-panics") , ("unused-tuple-struct-fields" , "dead-code") , ("static-mut-ref" , "static-mut-refs") ,] ,) , (Level :: Deny , & [("exceeding-bitshifts" , "arithmetic-overflow")]) ,] ;
};
}
