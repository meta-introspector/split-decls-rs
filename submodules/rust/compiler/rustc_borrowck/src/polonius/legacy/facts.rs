mkuse!{use std :: error :: Error ;}
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: fs :: { self , File } ;}
mkuse!{use std :: io :: Write ;}
mkuse!{use std :: path :: Path ;}
mkuse!{use polonius_engine :: { AllFacts , Atom , Output } ;}
mkuse!{use rustc_macros :: extension ;}
mkuse!{use rustc_middle :: mir :: Local ;}
mkuse!{use rustc_middle :: ty :: { RegionVid , TyCtxt } ;}
mkuse!{use rustc_mir_dataflow :: move_paths :: MovePathIndex ;}
mkuse!{use super :: { LocationIndex , PoloniusLocationTable } ;}
mkuse!{use crate :: BorrowIndex ;}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug)] pub struct RustcFacts ;}}
mkitem!{pub type PoloniusOutput = Output < RustcFacts > ;}
mkitem!{rustc_index :: newtype_index ! { # [doc = " A (kinda) newtype of `RegionVid` so we can implement `Atom` on it."] # [orderable] # [debug_format = "'?{}"] pub struct PoloniusRegionVid { } }}
mkitem!{mkimpl!{impl polonius_engine :: Atom for PoloniusRegionVid { fn index (self) -> usize { self . as_usize () } }}}
mkitem!{mkimpl!{impl From < RegionVid > for PoloniusRegionVid { fn from (value : RegionVid) -> Self { Self :: from_usize (value . as_usize ()) } }}}
mkitem!{mkimpl!{impl From < PoloniusRegionVid > for RegionVid { fn from (value : PoloniusRegionVid) -> Self { Self :: from_usize (value . as_usize ()) } }}}
mkitem!{mkimpl!{impl polonius_engine :: FactTypes for RustcFacts { type Origin = PoloniusRegionVid ; type Loan = BorrowIndex ; type Point = LocationIndex ; type Variable = Local ; type Path = MovePathIndex ; }}}
mkitem!{pub type PoloniusFacts = AllFacts < RustcFacts > ;}
mkitem!{mkimpl!{# [extension (pub (crate) trait PoloniusFactsExt)] impl PoloniusFacts { # [doc = " Returns `true` if there is a need to gather `PoloniusFacts` given the"] # [doc = " current `-Z` flags."] fn enabled (tcx : TyCtxt < '_ >) -> bool { tcx . sess . opts . unstable_opts . nll_facts || tcx . sess . opts . unstable_opts . polonius . is_legacy_enabled () } fn write_to_dir (& self , dir : impl AsRef < Path > , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { let dir : & Path = dir . as_ref () ; fs :: create_dir_all (dir) ? ; let wr = FactWriter { location_table , dir } ; macro_rules ! write_facts_to_path { ($ wr : ident . write_facts_to_path ($ this : ident . [$ ($ field : ident ,) *])) => { $ ($ wr . write_facts_to_path (&$ this .$ field , & format ! ("{}.facts" , stringify ! ($ field))) ?;) * } } write_facts_to_path ! { wr . write_facts_to_path (self . [loan_issued_at , universal_region , cfg_edge , loan_killed_at , subset_base , loan_invalidated_at , var_used_at , var_defined_at , var_dropped_at , use_of_var_derefs_origin , drop_of_var_derefs_origin , child_path , path_is_var , path_assigned_at_base , path_moved_at_base , path_accessed_at_base , known_placeholder_subset , placeholder ,]) } Ok (()) } }}}
mkitem!{mkimpl!{impl Atom for BorrowIndex { fn index (self) -> usize { self . as_usize () } }}}
mkitem!{mkimpl!{impl Atom for LocationIndex { fn index (self) -> usize { self . as_usize () } }}}
mkitem!{mkstruct!{struct FactWriter < 'w > { location_table : & 'w PoloniusLocationTable , dir : & 'w Path , }}}
mkitem!{mkimpl!{impl < 'w > FactWriter < 'w > { fn write_facts_to_path < T > (& self , rows : & [T] , file_name : & str) -> Result < () , Box < dyn Error > > where T : FactRow , { let file = & self . dir . join (file_name) ; let mut file = File :: create_buffered (file) ? ; for row in rows { row . write (& mut file , self . location_table) ? ; } Ok (()) } }}}
mkitem!{mktrait!{trait FactRow { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > ; }}}
mkitem!{mkimpl!{impl FactRow for PoloniusRegionVid { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [self]) } }}}
mkitem!{mkimpl!{impl < A , B > FactRow for (A , B) where A : FactCell , B : FactCell , { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [& self . 0 , & self . 1]) } }}}
mkitem!{mkimpl!{impl < A , B , C > FactRow for (A , B , C) where A : FactCell , B : FactCell , C : FactCell , { fn write (& self , out : & mut dyn Write , location_table : & PoloniusLocationTable ,) -> Result < () , Box < dyn Error > > { write_row (out , location_table , & [& self . 0 , & self . 1 , & self . 2]) } }}}

macro_rules! write_row_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_row in module {}", module_path!());
    };
}

mkfn!{
    write_row_introspect!();
    fn write_row (out : & mut dyn Write , location_table : & PoloniusLocationTable , columns : & [& dyn FactCell] ,) -> Result < () , Box < dyn Error > > { for (index , c) in columns . iter () . enumerate () { let tail = if index == columns . len () - 1 { "\n" } else { "\t" } ; write ! (out , "{:?}{tail}" , c . to_string (location_table)) ? ; } Ok (()) }
}
mkitem!{mktrait!{trait FactCell { fn to_string (& self , location_table : & PoloniusLocationTable) -> String ; }}}
mkitem!{mkimpl!{impl FactCell for BorrowIndex { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }}}
mkitem!{mkimpl!{impl FactCell for Local { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }}}
mkitem!{mkimpl!{impl FactCell for MovePathIndex { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }}}
mkitem!{mkimpl!{impl FactCell for PoloniusRegionVid { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }}}
mkitem!{mkimpl!{impl FactCell for RegionVid { fn to_string (& self , _location_table : & PoloniusLocationTable) -> String { format ! ("{self:?}") } }}}
mkitem!{mkimpl!{impl FactCell for LocationIndex { fn to_string (& self , location_table : & PoloniusLocationTable) -> String { format ! ("{:?}" , location_table . to_rich_location (* self)) } }}}