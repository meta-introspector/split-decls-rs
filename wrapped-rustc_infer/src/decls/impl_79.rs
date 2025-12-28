macro_rules! deps {
    () => {
        UndoLog!();
        OpaqueTypeTable!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'a , 'tcx > OpaqueTypeTable < 'a , 'tcx > { # [instrument (skip (self) , level = "debug")] pub fn register (& mut self , key : OpaqueTypeKey < 'tcx > , hidden_type : OpaqueHiddenType < 'tcx > ,) -> Option < Ty < 'tcx > > { if let Some (entry) = self . storage . opaque_types . get_mut (& key) { let prev = std :: mem :: replace (entry , hidden_type) ; self . undo_log . push (UndoLog :: OpaqueTypes (key , Some (prev))) ; return Some (prev . ty) ; } self . storage . opaque_types . insert (key , hidden_type) ; self . undo_log . push (UndoLog :: OpaqueTypes (key , None)) ; None } pub fn add_duplicate (& mut self , key : OpaqueTypeKey < 'tcx > , hidden_type : OpaqueHiddenType < 'tcx >) { self . storage . duplicate_entries . push ((key , hidden_type)) ; self . undo_log . push (UndoLog :: DuplicateOpaqueType) ; } }
    };
}

impl_79!()