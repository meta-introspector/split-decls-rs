/* FP:facts.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0001
/* FP:facts.rs-0002 */ use std :: error :: Error ;
/* FP:facts.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0002
/* FP:facts.rs-0004 */ use std :: fmt :: Debug ;
/* FP:facts.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0003
/* FP:facts.rs-0006 */ use std :: fs :: { self , File } ;
/* FP:facts.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0004
/* FP:facts.rs-0008 */ use std :: io :: Write ;
/* FP:facts.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0005
/* FP:facts.rs-0010 */ use std :: path :: Path ;
/* FP:facts.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0006
/* FP:facts.rs-0012 */ use polonius_engine :: { AllFacts , Atom , Output } ;
/* FP:facts.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0007
/* FP:facts.rs-0014 */ use rustc_macros :: extension ;
/* FP:facts.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0008
/* FP:facts.rs-0016 */ use crate :: rustc_complete :: mir :: Local ;
/* FP:facts.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0009
/* FP:facts.rs-0018 */ use crate :: rustc_complete :: ty :: { RegionVid , TyCtxt } ;
/* FP:facts.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0010
/* FP:facts.rs-0020 */ use crate :: rustc_mir_dataflow :: move_paths :: MovePathIndex ;
/* FP:facts.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0011
/* FP:facts.rs-0022 */ use super :: { LocationIndex , PoloniusLocationTable } ;
/* FP:facts.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_USE_0012
/* FP:facts.rs-0024 */ use crate :: BorrowIndex ;
/* FP:facts.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_STRUCT_0013
/* FP:facts.rs-0026 */ # [derive (Copy , Clone , Debug)] pub struct RustcFacts ;
/* FP:facts.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_TYPE_0014
/* FP:facts.rs-0028 */ pub type PoloniusOutput = Output < RustcFacts > ;
/* FP:facts.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_MACRO_0015
/* FP:facts.rs-0030 */ crate :: rustc_index :: newtype_index ! { # [doc = " A (kinda) newtype of `RegionVid` so we can implement `Atom` on it."] # [orderable] # [debug_format = "'?{}"] pub struct PoloniusRegionVid { } }
/* FP:facts.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0016
/* FP:facts.rs-0032 */ impl polonius_engine :: Atom for PoloniusRegionVid { fn index (self) -> usize { self . as_usize () } }
/* FP:facts.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0017
/* FP:facts.rs-0034 */ impl From < RegionVid > for PoloniusRegionVid { fn from (value : RegionVid) -> Self { Self :: from_usize (value . as_usize ()) } }
/* FP:facts.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0018
/* FP:facts.rs-0036 */ impl From < PoloniusRegionVid > for RegionVid { fn from (value : PoloniusRegionVid) -> Self { Self :: from_usize (value . as_usize ()) } }
/* FP:facts.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0019
/* FP:facts.rs-0038 */ impl polonius_engine :: FactTypes for RustcFacts { type Origin = PoloniusRegionVid ; type Loan = BorrowIndex ; type Point = LocationIndex ; type Variable = Local ; type Path = MovePathIndex ; }
/* FP:facts.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_TYPE_0020
/* FP:facts.rs-0040 */ pub type PoloniusFacts = AllFacts < RustcFacts > ;
/* FP:facts.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0021
/* FP:facts.rs-0042 */ # [extension (pub (crate) trait PoloniusFactsExt)] impl PoloniusFacts { # [doc = " Returns `true` if there is a need to gather `PoloniusFacts` given the"] # [doc = " current `-Z` flags."] fn enabled (tcx : TyCtxt < '_ >) -> bool { tcx . sess . opts . unstable_opts . nll_facts || tcx . sess . opts . unstable_opts . polonius . is_legacy_enabled () } fn write_to_dir (& self , dir : impl AsRef < Path > , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { let dir : & Path = dir . as_ref () ; fs :: create_dir_all (dir) ? ; let wr = FactWriter { location_table , dir } ; macro_rules ! write_facts_to_path { ($ wr : ident . write_facts_to_path ($ this : ident . [$ ($ field : ident ,) *])) => { $ ($ wr . write_facts_to_path (&$ this .$ field , & format ! ("{}.facts" , stringify ! ($ field))) ?;) * } } write_facts_to_path ! { wr . write_facts_to_path (self . [loan_issued_at , universal_region , cfg_edge , loan_killed_at , subset_base , loan_invalidated_at , var_used_at , var_defined_at , var_dropped_at , use_of_var_derefs_origin , drop_of_var_derefs_origin , child_path , path_is_var , path_assigned_at_base , path_moved_at_base , path_accessed_at_base , known_placeholder_subset , placeholder ,]) } Ok (()) } }
/* FP:facts.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0022
/* FP:facts.rs-0044 */ impl Atom for BorrowIndex { fn index (self) -> usize { self . as_usize () } }
/* FP:facts.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0023
/* FP:facts.rs-0046 */ impl Atom for LocationIndex { fn index (self) -> usize { self . as_usize () } }
/* FP:facts.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_STRUCT_0024
/* FP:facts.rs-0048 */ struct FactWriter < 'w > { location_table : & 'w PoloniusLocationTable , dir : & 'w Path , }
/* FP:facts.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0025
/* FP:facts.rs-0050 */ impl < 'w > FactWriter < 'w > { fn write_facts_to_path < T > (& self , rows : & [T] , file_name : & str) -> Result < () , Box < dyn Error > > where T : FactRow , { let file = & self . dir . join (file_name) ; let mut file = File :: create_buffered (file) ? ; for row in rows { row . write (& mut file , self . location_table) ? ; } Ok (()) } }
/* FP:facts.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_TRAIT_0026
/* FP:facts.rs-0052 */ trait FactRow { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > ; }
/* FP:facts.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0027
/* FP:facts.rs-0054 */ impl FactRow for PoloniusRegionVid { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [self]) } }
/* FP:facts.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0028
/* FP:facts.rs-0056 */ impl < A , B > FactRow for (A , B) where A : FactCell , B : FactCell , { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [& self . 0 , & self . 1]) } }
/* FP:facts.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0029
/* FP:facts.rs-0058 */ impl < A , B , C > FactRow for (A , B , C) where A : FactCell , B : FactCell , C : FactCell , { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [& self . 0 , & self . 1 , & self . 2]) } }
/* FP:facts.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_FN_0030
/* FP:facts.rs-0060 */ fn write_row (out : & mut dyn Write , location_table : & PoloniusLocationTable , columns : & [& dyn FactCell] ,) -> Result < () , Box < dyn Error > > { for (index , c) in columns . iter () . enumerate () { let tail = if index == columns . len () - 1 { "\n" } else { "\t" } ; write ! (out , "{:?}{tail}" , c . to_string (location_table)) ? ; } Ok (()) }
/* FP:facts.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_TRAIT_0031
/* FP:facts.rs-0062 */ trait FactCell { fn to_string (& self , location_table : & PoloniusLocationTable) -> String ; }
/* FP:facts.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0032
/* FP:facts.rs-0064 */ impl FactCell for BorrowIndex { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }
/* FP:facts.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0033
/* FP:facts.rs-0066 */ impl FactCell for Local { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }
/* FP:facts.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0034
/* FP:facts.rs-0068 */ impl FactCell for MovePathIndex { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }
/* FP:facts.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0035
/* FP:facts.rs-0070 */ impl FactCell for PoloniusRegionVid { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }
/* FP:facts.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0036
/* FP:facts.rs-0072 */ impl FactCell for RegionVid { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }
/* FP:facts.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_polonius_legacy_facts_IMPL_0037
/* FP:facts.rs-0074 */ impl FactCell for LocationIndex { fn to_string (& self , location_table : & PoloniusLocationTable) -> String { format ! ("{:?}" , location_table . to_rich_location (* self)) } }