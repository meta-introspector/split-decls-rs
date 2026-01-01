/* FP:locals.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_USE_0001
/* FP:locals.rs-0002 */ use std :: ops :: { Index , IndexMut } ;
/* FP:locals.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_USE_0002
/* FP:locals.rs-0004 */ use crate :: rustc_index :: IndexVec ;
/* FP:locals.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_USE_0003
/* FP:locals.rs-0006 */ use crate :: rustc_complete :: mir ;
/* FP:locals.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_USE_0004
/* FP:locals.rs-0008 */ use crate :: rustc_complete :: ty :: print :: with_no_trimmed_paths ;
/* FP:locals.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_USE_0005
/* FP:locals.rs-0010 */ use tracing :: { debug , warn } ;
/* FP:locals.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_USE_0006
/* FP:locals.rs-0012 */ use crate :: mir :: { FunctionCx , LocalRef } ;
/* FP:locals.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_USE_0007
/* FP:locals.rs-0014 */ use crate :: traits :: BuilderMethods ;
/* FP:locals.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_STRUCT_0008
/* FP:locals.rs-0016 */ pub (super) struct Locals < 'tcx , V > { values : IndexVec < mir :: Local , LocalRef < 'tcx , V > > , }
/* FP:locals.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_IMPL_0009
/* FP:locals.rs-0018 */ impl < 'tcx , V > Index < mir :: Local > for Locals < 'tcx , V > { type Output = LocalRef < 'tcx , V > ; # [inline] fn index (& self , index : mir :: Local) -> & LocalRef < 'tcx , V > { & self . values [index] } }
/* FP:locals.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_IMPL_0010
/* FP:locals.rs-0020 */ # [doc = " To mutate locals, use `FunctionCx::overwrite_local` instead."] impl < 'tcx , V , Idx : ? Sized > ! IndexMut < Idx > for Locals < 'tcx , V > { }
/* FP:locals.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_IMPL_0011
/* FP:locals.rs-0022 */ impl < 'tcx , V > Locals < 'tcx , V > { pub (super) fn empty () -> Locals < 'tcx , V > { Locals { values : IndexVec :: default () } } pub (super) fn indices (& self) -> impl DoubleEndedIterator < Item = mir :: Local > + Clone + 'tcx { self . values . indices () } }
/* FP:locals.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_mir_locals_IMPL_0012
/* FP:locals.rs-0024 */ impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { pub (super) fn initialize_locals (& mut self , values : Vec < LocalRef < 'tcx , Bx :: Value > >) { assert ! (self . locals . values . is_empty ()) ; for (local , value) in values . into_iter () . enumerate () { match value { LocalRef :: Place (_) | LocalRef :: UnsizedPlace (_) | LocalRef :: PendingOperand => () , LocalRef :: Operand (op) => { let local = mir :: Local :: from_usize (local) ; let expected_ty = self . monomorphize (self . mir . local_decls [local] . ty) ; if expected_ty != op . layout . ty { warn ! ("Unexpected initial operand type:\nexpected {expected_ty:?},\nfound    {:?}.\n\
/* FP:locals.rs-0025 */                             See <https://github.com/rust-lang/rust/issues/114858>." , op . layout . ty) ; } } } self . locals . values . push (value) ; } } pub (super) fn overwrite_local (& mut self , local : mir :: Local , mut value : LocalRef < 'tcx , Bx :: Value > ,) { match value { LocalRef :: Place (_) | LocalRef :: UnsizedPlace (_) | LocalRef :: PendingOperand => () , LocalRef :: Operand (ref mut op) => { let local_ty = self . monomorphize (self . mir . local_decls [local] . ty) ; if local_ty != op . layout . ty { debug ! ("updating type of operand due to subtyping") ; with_no_trimmed_paths ! (debug ! (? op . layout . ty)) ; with_no_trimmed_paths ! (debug ! (? local_ty)) ; op . layout . ty = local_ty ; } } } ; self . locals . values [local] = value ; } }