// Generated macro for builtin_deref (function)
macro_rules! Depcrate_autoderefbuiltin_deref {
() => {
// Module: crate::autoderef
// Provides: {"builtin_deref"}
// Dependencies: {}
pub (crate) fn builtin_deref < 'ty > (db : & dyn HirDatabase , ty : & 'ty Ty , explicit : bool ,) -> Option < & 'ty Ty > { match ty . kind (Interner) { TyKind :: Ref (.. , ty) => Some (ty) , TyKind :: Raw (.. , ty) if explicit => Some (ty) , & TyKind :: Adt (chalk_ir :: AdtId (adt) , ref substs) if crate :: lang_items :: is_box (db , adt) => { substs . at (Interner , 0) . ty (Interner) } _ => None , } }
};
}
