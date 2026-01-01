/* FP:pattern.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_USE_0001
/* FP:pattern.rs-0002 */ use std :: fmt ;
/* FP:pattern.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_USE_0002
/* FP:pattern.rs-0004 */ use crate :: rustc_data_structures :: intern :: Interned ;
/* FP:pattern.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_USE_0003
/* FP:pattern.rs-0006 */ use rustc_macros :: HashStable ;
/* FP:pattern.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_USE_0004
/* FP:pattern.rs-0008 */ use rustc_type_ir :: ir_print :: IrPrint ;
/* FP:pattern.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_USE_0005
/* FP:pattern.rs-0010 */ use rustc_type_ir :: { FlagComputation , Flags , { self as ir } , } ;
/* FP:pattern.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_USE_0006
/* FP:pattern.rs-0012 */ use super :: TyCtxt ;
/* FP:pattern.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_USE_0007
/* FP:pattern.rs-0014 */ use crate :: ty ;
/* FP:pattern.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_TYPE_0008
/* FP:pattern.rs-0016 */ pub type PatternKind < 'tcx > = ir :: PatternKind < TyCtxt < 'tcx > > ;
/* FP:pattern.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_STRUCT_0009
/* FP:pattern.rs-0018 */ # [derive (Copy , Clone , PartialEq , Eq , Hash , HashStable)] # [rustc_pass_by_value] pub struct Pattern < 'tcx > (pub Interned < 'tcx , PatternKind < 'tcx > >) ;
/* FP:pattern.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_IMPL_0010
/* FP:pattern.rs-0020 */ impl < 'tcx > Flags for Pattern < 'tcx > { fn flags (& self) -> rustc_type_ir :: TypeFlags { match & * * self { ty :: PatternKind :: Range { start , end } => { FlagComputation :: for_const_kind (& start . kind ()) . flags | FlagComputation :: for_const_kind (& end . kind ()) . flags } ty :: PatternKind :: Or (pats) => { let mut flags = pats [0] . flags () ; for pat in pats [1 ..] . iter () { flags |= pat . flags () ; } flags } } } fn outer_exclusive_binder (& self) -> rustc_type_ir :: DebruijnIndex { match & * * self { ty :: PatternKind :: Range { start , end } => { start . outer_exclusive_binder () . max (end . outer_exclusive_binder ()) } ty :: PatternKind :: Or (pats) => { let mut idx = pats [0] . outer_exclusive_binder () ; for pat in pats [1 ..] . iter () { idx = idx . max (pat . outer_exclusive_binder ()) ; } idx } } } }
/* FP:pattern.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_IMPL_0011
/* FP:pattern.rs-0022 */ impl < 'tcx > std :: ops :: Deref for Pattern < 'tcx > { type Target = PatternKind < 'tcx > ; fn deref (& self) -> & Self :: Target { & * self . 0 } }
/* FP:pattern.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_IMPL_0012
/* FP:pattern.rs-0024 */ impl < 'tcx > fmt :: Debug for Pattern < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:?}" , ** self) } }
/* FP:pattern.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_IMPL_0013
/* FP:pattern.rs-0026 */ impl < 'tcx > IrPrint < PatternKind < 'tcx > > for TyCtxt < 'tcx > { fn print (t : & PatternKind < 'tcx > , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * t { PatternKind :: Range { start , end } => { write ! (f , "{start}") ? ; if let Some (c) = end . try_to_value () { let end = c . valtree . unwrap_leaf () ; let size = end . size () ; let max = match c . ty . kind () { ty :: Int (_) => { Some (ty :: ScalarInt :: truncate_from_int (size . signed_int_max () , size)) } ty :: Uint (_) => { Some (ty :: ScalarInt :: truncate_from_uint (size . unsigned_int_max () , size)) } ty :: Char => Some (ty :: ScalarInt :: truncate_from_uint (char :: MAX , size)) , _ => None , } ; if let Some ((max , _)) = max && end == max { return write ! (f , "..") ; } } write ! (f , "..={end}") } PatternKind :: Or (patterns) => { write ! (f , "(") ? ; let mut first = true ; for pat in patterns { if first { first = false } else { write ! (f , " | ") ? ; } write ! (f , "{pat:?}") ? ; } write ! (f , ")") } } } fn print_debug (t : & PatternKind < 'tcx > , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { Self :: print (t , fmt) } }
/* FP:pattern.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_pattern_IMPL_0014
/* FP:pattern.rs-0028 */ impl < 'tcx > rustc_type_ir :: inherent :: IntoKind for Pattern < 'tcx > { type Kind = PatternKind < 'tcx > ; fn kind (self) -> Self :: Kind { * self } }