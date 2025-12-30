// Generated macro for Safety (enum)
macro_rules! Depcrate_astSafety {
() => {
// Module: crate::ast
// Provides: {"Safety"}
// Dependencies: {}
# [doc = " Safety of items."] # [derive (Copy , Clone , PartialEq , Eq , Hash , Encodable , Decodable , Debug)] # [derive (HashStable_Generic , Walkable)] pub enum Safety { # [doc = " `unsafe` an item is explicitly marked as `unsafe`."] Unsafe (Span) , # [doc = " `safe` an item is explicitly marked as `safe`."] Safe (Span) , # [doc = " Default means no value was provided, it will take a default value given the context in"] # [doc = " which is used."] Default , }
};
}
