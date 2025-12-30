// Generated macro for tests (module)
macro_rules! Depcrate_stmttests {
() => {
// Module: crate::stmt
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_superclasses_required_items () { let superclasses = [ItemIdentifier :: dummy (1) , ItemIdentifier :: dummy (2) , ItemIdentifier :: dummy (3) ,] ; let required_items = [ItemTree :: new (ItemIdentifier :: dummy (1) , [ItemTree :: new (ItemIdentifier :: dummy (2) , [ItemTree :: new (ItemIdentifier :: dummy (3) , [ItemTree :: objc ("__macros__")]) , ItemTree :: objc ("__macros__") ,] ,) , ItemTree :: new (ItemIdentifier :: dummy (3) , [ItemTree :: objc ("__macros__")]) , ItemTree :: objc ("__macros__") ,] ,) , ItemTree :: new (ItemIdentifier :: dummy (2) , [ItemTree :: new (ItemIdentifier :: dummy (3) , [ItemTree :: objc ("__macros__")]) , ItemTree :: objc ("__macros__") ,] ,) , ItemTree :: new (ItemIdentifier :: dummy (3) , [ItemTree :: objc ("__macros__")]) , ItemTree :: objc ("__macros__") ,] ; assert_eq ! (superclasses_required_items (superclasses) . collect ::< Vec < _ >> () , required_items) ; } }
};
}
