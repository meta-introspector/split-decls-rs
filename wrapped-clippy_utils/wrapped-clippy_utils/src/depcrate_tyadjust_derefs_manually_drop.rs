// Generated macro for adjust_derefs_manually_drop (function)
macro_rules! Depcrate_tyadjust_derefs_manually_drop {
() => {
// Module: crate::ty
// Provides: {"adjust_derefs_manually_drop"}
// Dependencies: {}
# [doc = " Checks if the adjustments contain a mutable dereference of a `ManuallyDrop<_>`."] pub fn adjust_derefs_manually_drop < 'tcx > (adjustments : & 'tcx [Adjustment < 'tcx >] , mut ty : Ty < 'tcx >) -> bool { adjustments . iter () . any (| a | { let ty = mem :: replace (& mut ty , a . target) ; matches ! (a . kind , Adjust :: Deref (Some (op)) if op . mutbl == Mutability :: Mut) && is_manually_drop (ty) }) }
};
}
