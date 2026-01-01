/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_USE_0001
/* FP:mod.rs-0002 */ use std :: fmt ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_USE_0002
/* FP:mod.rs-0004 */ use std :: ops :: Index ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_USE_0003
/* FP:mod.rs-0006 */ use crate :: rustc_index :: { IndexSlice , IndexVec } ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_USE_0004
/* FP:mod.rs-0008 */ use crate :: rustc_complete :: mir :: ConstraintCategory ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_USE_0005
/* FP:mod.rs-0010 */ use crate :: rustc_complete :: ty :: { RegionVid , TyCtxt , VarianceDiagInfo } ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_USE_0006
/* FP:mod.rs-0012 */ use crate :: rustc_complete :: Span ;
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_USE_0007
/* FP:mod.rs-0014 */ use tracing :: debug ;
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_USE_0008
/* FP:mod.rs-0016 */ use crate :: type_check :: Locations ;
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_MOD_0009
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_STRUCT_0010
/* FP:mod.rs-0020 */ # [doc = " A set of NLL region constraints. These include \"outlives\""] # [doc = " constraints of the form `R1: R2`. Each constraint is identified by"] # [doc = " a unique `OutlivesConstraintIndex` and you can index into the set"] # [doc = " (`constraint_set[i]`) to access the constraint details."] # [derive (Clone , Debug , Default)] pub (crate) struct OutlivesConstraintSet < 'tcx > { outlives : IndexVec < OutlivesConstraintIndex , OutlivesConstraint < 'tcx > > , }
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_IMPL_0011
/* FP:mod.rs-0022 */ impl < 'tcx > OutlivesConstraintSet < 'tcx > { pub (crate) fn push (& mut self , constraint : OutlivesConstraint < 'tcx >) { debug ! ("OutlivesConstraintSet::push({:?})" , constraint) ; if constraint . sup == constraint . sub { return ; } self . outlives . push (constraint) ; } # [doc = " Constructs a \"normal\" graph from the constraint set; the graph makes it"] # [doc = " easy to find the constraints affecting a particular region."] # [doc = ""] # [doc = " N.B., this graph contains a \"frozen\" view of the current"] # [doc = " constraints. Any new constraints added to the `OutlivesConstraintSet`"] # [doc = " after the graph is built will not be present in the graph."] pub (crate) fn graph (& self , num_region_vars : usize) -> graph :: NormalConstraintGraph { graph :: ConstraintGraph :: new (graph :: Normal , self , num_region_vars) } # [doc = " Like `graph`, but constraints a reverse graph where `R1: R2`"] # [doc = " represents an edge `R2 -> R1`."] pub (crate) fn reverse_graph (& self , num_region_vars : usize) -> graph :: ReverseConstraintGraph { graph :: ConstraintGraph :: new (graph :: Reverse , self , num_region_vars) } pub (crate) fn outlives (& self ,) -> & IndexSlice < OutlivesConstraintIndex , OutlivesConstraint < 'tcx > > { & self . outlives } }
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_IMPL_0012
/* FP:mod.rs-0024 */ impl < 'tcx > Index < OutlivesConstraintIndex > for OutlivesConstraintSet < 'tcx > { type Output = OutlivesConstraint < 'tcx > ; fn index (& self , i : OutlivesConstraintIndex) -> & Self :: Output { & self . outlives [i] } }
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_STRUCT_0013
/* FP:mod.rs-0026 */ # [derive (Copy , Clone , PartialEq , Eq)] pub struct OutlivesConstraint < 'tcx > { # [doc = " The region SUP must outlive SUB..."] pub sup : RegionVid , # [doc = " Region that must be outlived."] pub sub : RegionVid , # [doc = " Where did this constraint arise?"] pub locations : Locations , # [doc = " The `Span` associated with the creation of this constraint."] # [doc = " This should be used in preference to obtaining the span from"] # [doc = " `locations`, since the `locations` may give a poor span"] # [doc = " in some cases (e.g. converting a constraint from a promoted)."] pub span : Span , # [doc = " What caused this constraint?"] pub category : ConstraintCategory < 'tcx > , # [doc = " Variance diagnostic information"] pub variance_info : VarianceDiagInfo < TyCtxt < 'tcx > > , # [doc = " If this constraint is promoted from closure requirements."] pub from_closure : bool , }
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_IMPL_0014
/* FP:mod.rs-0028 */ impl < 'tcx > fmt :: Debug for OutlivesConstraint < 'tcx > { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "({:?}: {:?}) due to {:?} ({:?}) ({:?})" , self . sup , self . sub , self . locations , self . variance_info , self . category ,) } }
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_MACRO_0015
/* FP:mod.rs-0030 */ crate :: rustc_index :: newtype_index ! { # [debug_format = "OutlivesConstraintIndex({})"] pub (crate) struct OutlivesConstraintIndex { } }
/* FP:mod.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_constraints_mod_MACRO_0016
/* FP:mod.rs-0032 */ crate :: rustc_index :: newtype_index ! { # [orderable] # [debug_format = "ConstraintSccIndex({})"] pub struct ConstraintSccIndex { } }