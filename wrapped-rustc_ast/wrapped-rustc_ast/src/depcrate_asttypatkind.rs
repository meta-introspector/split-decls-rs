// Generated macro for TyPatKind (enum)
macro_rules! Depcrate_astTyPatKind {
() => {
// Module: crate::ast
// Provides: {"TyPatKind"}
// Dependencies: {}
# [doc = " All the different flavors of pattern that Rust recognizes."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum TyPatKind { # [doc = " A range pattern (e.g., `1...2`, `1..2`, `1..`, `..2`, `1..=2`, `..=2`)."] Range (Option < Box < AnonConst > > , Option < Box < AnonConst > > , Spanned < RangeEnd >) , Or (ThinVec < Box < TyPat > >) , # [doc = " Placeholder for a pattern that wasn't syntactically well formed in some way."] Err (ErrorGuaranteed) , }
};
}
