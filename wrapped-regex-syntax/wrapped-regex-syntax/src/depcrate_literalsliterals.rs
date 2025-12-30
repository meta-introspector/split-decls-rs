// Generated macro for Literals (struct)
macro_rules! Depcrate_literalsLiterals {
() => {
// Module: crate::literals
// Provides: {"Literals"}
// Dependencies: {}
# [doc = " A set of literal byte strings extracted from a regular expression."] # [doc = ""] # [doc = " Every member of the set is a `Lit`, which is represented by a `Vec<u8>`."] # [doc = " (Notably, it may contain invalid UTF-8.) Every member is said to be either"] # [doc = " *complete* or *cut*. A complete literal means that it extends until the"] # [doc = " beginning (or end) of the regular expression. In some circumstances, this"] # [doc = " can be used to indicate a match in the regular expression."] # [doc = ""] # [doc = " Note that a key aspect of literal extraction is knowing when to stop. It is"] # [doc = " not feasible to blindly extract all literals from a regular expression,"] # [doc = " even if there are finitely many. For example, the regular expression"] # [doc = " `[0-9]{10}` has `10^10` distinct literals. For this reason, literal"] # [doc = " extraction is bounded to some low number by default using heuristics, but"] # [doc = " the limits can be tweaked."] # [derive (Clone , Eq , PartialEq)] pub struct Literals { lits : Vec < Lit > , limit_size : usize , limit_class : usize , }
};
}
