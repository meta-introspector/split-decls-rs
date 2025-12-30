// Generated macro for ItemTree (struct)
macro_rules! Depcrate_idItemTree {
() => {
// Module: crate::id
// Provides: {"ItemTree"}
// Dependencies: {}
# [doc = " Items relative to their \"super\" item, forming a DAG, potentially spanning"] # [doc = " many different libraries/frameworks."] # [doc = ""] # [doc = " Allows us to emit e.g.:"] # [doc = " XYZ = [\"objc2-foundation/NSGeometry\", \"objc2-foundation/objc2-core-foundation\"]"] # [doc = ""] # [doc = " Instead of:"] # [doc = " XYZ = [\"objc2-foundation/NSGeometry\", \"objc2-core-foundation\"]"] # [derive (Debug , Clone)] pub struct ItemTree { id : ItemIdentifier , children : BTreeSet < ItemTree > , }
};
}
