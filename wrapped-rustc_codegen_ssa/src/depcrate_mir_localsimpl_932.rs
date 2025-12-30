// Generated macro for impl_932 (impl)
macro_rules! Depcrate_mir_localsimpl_932 {
() => {
// Module: crate::mir::locals
// Provides: {"impl_932"}
// Dependencies: {}
impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { pub (super) fn initialize_locals (& mut self , values : Vec < LocalRef < 'tcx , Bx :: Value > >) { assert ! (self . locals . values . is_empty ()) ; for (local , value) in values . into_iter () . enumerate () { match value { LocalRef :: Place (_) | LocalRef :: UnsizedPlace (_) | LocalRef :: PendingOperand => () , LocalRef :: Operand (op) => { let local = mir :: Local :: from_usize (local) ; let expected_ty = self . monomorphize (self . mir . local_decls [local] . ty) ; if expected_ty != op . layout . ty { warn ! ("Unexpected initial operand type:\nexpected {expected_ty:?},\nfound    {:?}.\n\
                            See <https://github.com/rust-lang/rust/issues/114858>." , op . layout . ty) ; } } } self . locals . values . push (value) ; } } pub (super) fn overwrite_local (& mut self , local : mir :: Local , mut value : LocalRef < 'tcx , Bx :: Value > ,) { match value { LocalRef :: Place (_) | LocalRef :: UnsizedPlace (_) | LocalRef :: PendingOperand => () , LocalRef :: Operand (ref mut op) => { let local_ty = self . monomorphize (self . mir . local_decls [local] . ty) ; if local_ty != op . layout . ty { debug ! ("updating type of operand due to subtyping") ; with_no_trimmed_paths ! (debug ! (? op . layout . ty)) ; with_no_trimmed_paths ! (debug ! (? local_ty)) ; op . layout . ty = local_ty ; } } } ; self . locals . values [local] = value ; } }
};
}
