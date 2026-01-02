mkuse!{use rustc_errors :: { Diag , ErrorGuaranteed } ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use crate :: error_reporting :: TypeErrCtxt ;}
mkuse!{use crate :: infer :: RegionResolutionError ;}
mkmod!{different_lifetimes, { 
                getname!(different_lifetimes);
                getsrc!(different_lifetimes);
                getpath!(different_lifetimes);
                get_deps!(different_lifetimes);
                get_crates!(different_lifetimes);
                mkinclude!(different_lifetimes);
                 
            }}
mkmod!{find_anon_type, { 
                getname!(find_anon_type);
                getsrc!(find_anon_type);
                getpath!(find_anon_type);
                get_deps!(find_anon_type);
                get_crates!(find_anon_type);
                mkinclude!(find_anon_type);
                 
            }}
mkmod!{mismatched_static_lifetime, { 
                getname!(mismatched_static_lifetime);
                getsrc!(mismatched_static_lifetime);
                getpath!(mismatched_static_lifetime);
                get_deps!(mismatched_static_lifetime);
                get_crates!(mismatched_static_lifetime);
                mkinclude!(mismatched_static_lifetime);
                 
            }}
mkmod!{named_anon_conflict, { 
                getname!(named_anon_conflict);
                getsrc!(named_anon_conflict);
                getpath!(named_anon_conflict);
                get_deps!(named_anon_conflict);
                get_crates!(named_anon_conflict);
                mkinclude!(named_anon_conflict);
                 
            }}
mkmod!{placeholder_error, { 
                getname!(placeholder_error);
                getsrc!(placeholder_error);
                getpath!(placeholder_error);
                get_deps!(placeholder_error);
                get_crates!(placeholder_error);
                mkinclude!(placeholder_error);
                 
            }}
mkmod!{placeholder_relation, { 
                getname!(placeholder_relation);
                getsrc!(placeholder_relation);
                getpath!(placeholder_relation);
                get_deps!(placeholder_relation);
                get_crates!(placeholder_relation);
                mkinclude!(placeholder_relation);
                 
            }}
mkmod!{static_impl_trait, { 
                getname!(static_impl_trait);
                getsrc!(static_impl_trait);
                getpath!(static_impl_trait);
                get_deps!(static_impl_trait);
                get_crates!(static_impl_trait);
                mkinclude!(static_impl_trait);
                 
            }}
mkmod!{trait_impl_difference, { 
                getname!(trait_impl_difference);
                getsrc!(trait_impl_difference);
                getpath!(trait_impl_difference);
                get_deps!(trait_impl_difference);
                get_crates!(trait_impl_difference);
                mkinclude!(trait_impl_difference);
                 
            }}
mkmod!{util, { 
                getname!(util);
                getsrc!(util);
                getpath!(util);
                get_deps!(util);
                get_crates!(util);
                mkinclude!(util);
                 
            }}
mkuse!{pub use different_lifetimes :: suggest_adding_lifetime_params ;}
mkuse!{pub use find_anon_type :: find_anon_type ;}
mkuse!{pub use static_impl_trait :: { HirTraitObjectVisitor , TraitObjectVisitor , suggest_new_region_bound } ;}
mkuse!{pub use util :: find_param_with_region ;}
mkitem!{mkimpl!{impl < 'cx , 'tcx > TypeErrCtxt < 'cx , 'tcx > { pub fn try_report_nice_region_error (& 'cx self , generic_param_scope : LocalDefId , error : & RegionResolutionError < 'tcx > ,) -> Option < ErrorGuaranteed > { NiceRegionError :: new (self , generic_param_scope , error . clone ()) . try_report () } }}}
mkitem!{mkstruct!{pub struct NiceRegionError < 'cx , 'tcx > { cx : & 'cx TypeErrCtxt < 'cx , 'tcx > , # [doc = " The innermost definition that introduces generic parameters that may be involved in"] # [doc = " the region errors we are dealing with."] generic_param_scope : LocalDefId , error : Option < RegionResolutionError < 'tcx > > , regions : Option < (Span , ty :: Region < 'tcx > , ty :: Region < 'tcx >) > , }}}
mkitem!{mkimpl!{impl < 'cx , 'tcx > NiceRegionError < 'cx , 'tcx > { pub fn new (cx : & 'cx TypeErrCtxt < 'cx , 'tcx > , generic_param_scope : LocalDefId , error : RegionResolutionError < 'tcx > ,) -> Self { Self { cx , error : Some (error) , regions : None , generic_param_scope } } pub fn new_from_span (cx : & 'cx TypeErrCtxt < 'cx , 'tcx > , generic_param_scope : LocalDefId , span : Span , sub : ty :: Region < 'tcx > , sup : ty :: Region < 'tcx > ,) -> Self { Self { cx , error : None , regions : Some ((span , sub , sup)) , generic_param_scope } } fn tcx (& self) -> TyCtxt < 'tcx > { self . cx . tcx } pub fn try_report_from_nll (& self) -> Option < Diag < 'tcx > > { self . try_report_named_anon_conflict () . or_else (| | self . try_report_placeholder_conflict ()) . or_else (| | self . try_report_placeholder_relation ()) } pub fn try_report (& self) -> Option < ErrorGuaranteed > { self . try_report_from_nll () . map (| diag | diag . emit ()) . or_else (| | self . try_report_impl_not_conforming_to_trait ()) . or_else (| | self . try_report_anon_anon_conflict ()) . or_else (| | self . try_report_static_impl_trait ()) . or_else (| | self . try_report_mismatched_static_lifetime ()) } pub (super) fn regions (& self) -> Option < (Span , ty :: Region < 'tcx > , ty :: Region < 'tcx >) > { match (& self . error , self . regions) { (Some (RegionResolutionError :: ConcreteFailure (origin , sub , sup)) , None) => { Some ((origin . span () , * sub , * sup)) } (Some (RegionResolutionError :: SubSupConflict (_ , _ , origin , sub , _ , sup , _)) , None) => { Some ((origin . span () , * sub , * sup)) } (None , Some ((span , sub , sup))) => Some ((span , sub , sup)) , _ => None , } } }}}