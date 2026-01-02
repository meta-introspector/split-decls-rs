mkuse!{use rustc_hir :: attrs :: { AttributeKind , CoverageAttrKind } ;}
mkuse!{use rustc_hir :: find_attr ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;}
mkuse!{use rustc_middle :: mir :: coverage :: { BasicCoverageBlock , CoverageIdsInfo , CoverageKind , MappingKind } ;}
mkuse!{use rustc_middle :: mir :: { Body , Statement , StatementKind } ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt } ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_span :: def_id :: LocalDefId ;}
mkuse!{use tracing :: trace ;}
mkuse!{use crate :: coverage :: counters :: node_flow :: make_node_counters ;}
mkuse!{use crate :: coverage :: counters :: { CoverageCounters , transcribe_counters } ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    # [doc = " Registers query/hook implementations related to coverage."] pub (crate) fn provide (providers : & mut Providers) { providers . hooks . is_eligible_for_coverage = is_eligible_for_coverage ; providers . queries . coverage_attr_on = coverage_attr_on ; providers . queries . coverage_ids_info = coverage_ids_info ; }
}

macro_rules! is_eligible_for_coverage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_eligible_for_coverage in module {}", module_path!());
    };
}

mkfn!{
    is_eligible_for_coverage_introspect!();
    # [doc = " Hook implementation for [`TyCtxt::is_eligible_for_coverage`]."] fn is_eligible_for_coverage (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { if ! tcx . def_kind (def_id) . is_fn_like () { trace ! ("InstrumentCoverage skipped for {def_id:?} (not an fn-like)") ; return false ; } if tcx . codegen_fn_attrs (def_id) . flags . contains (CodegenFnAttrFlags :: NAKED) { trace ! ("InstrumentCoverage skipped for {def_id:?} (`#[naked]`)") ; return false ; } if ! tcx . coverage_attr_on (def_id) { trace ! ("InstrumentCoverage skipped for {def_id:?} (`#[coverage(off)]`)") ; return false ; } true }
}

macro_rules! coverage_attr_on_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function coverage_attr_on in module {}", module_path!());
    };
}

mkfn!{
    coverage_attr_on_introspect!();
    # [doc = " Query implementation for `coverage_attr_on`."] fn coverage_attr_on (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { if let Some (kind) = find_attr ! (tcx . get_all_attrs (def_id) , AttributeKind :: Coverage (_sp , kind) => kind) { match kind { CoverageAttrKind :: On => return true , CoverageAttrKind :: Off => return false , } } ; if tcx . is_automatically_derived (def_id . to_def_id ()) { return false ; } match tcx . opt_local_parent (def_id) { Some (parent) => tcx . coverage_attr_on (parent) , None => true , } }
}

macro_rules! coverage_ids_info_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function coverage_ids_info in module {}", module_path!());
    };
}

mkfn!{
    coverage_ids_info_introspect!();
    # [doc = " Query implementation for `coverage_ids_info`."] fn coverage_ids_info < 'tcx > (tcx : TyCtxt < 'tcx > , instance_def : ty :: InstanceKind < 'tcx > ,) -> Option < CoverageIdsInfo > { let mir_body = tcx . instance_mir (instance_def) ; let fn_cov_info = mir_body . function_coverage_info . as_deref () ? ; let mut bcbs_seen = DenseBitSet :: new_empty (fn_cov_info . priority_list . len ()) ; for kind in all_coverage_in_mir_body (mir_body) { match * kind { CoverageKind :: VirtualCounter { bcb } => { bcbs_seen . insert (bcb) ; } _ => { } } } let mut bcb_needs_counter = DenseBitSet :: < BasicCoverageBlock > :: new_empty (fn_cov_info . priority_list . len ()) ; for mapping in & fn_cov_info . mappings { match mapping . kind { MappingKind :: Code { bcb } => { bcb_needs_counter . insert (bcb) ; } MappingKind :: Branch { true_bcb , false_bcb } => { bcb_needs_counter . insert (true_bcb) ; bcb_needs_counter . insert (false_bcb) ; } } } let mut priority_list = fn_cov_info . priority_list . clone () ; debug_assert_eq ! (priority_list [0] , priority_list . iter () . copied () . max () . unwrap ()) ; assert ! (! bcbs_seen . contains (priority_list [0])) ; priority_list [1 ..] . sort_by_key (| & bcb | ! bcbs_seen . contains (bcb)) ; let node_counters = make_node_counters (& fn_cov_info . node_flow_data , & priority_list) ; let coverage_counters = transcribe_counters (& node_counters , & bcb_needs_counter , & bcbs_seen) ; let CoverageCounters { phys_counter_for_node , next_counter_id , node_counters , expressions , .. } = coverage_counters ; Some (CoverageIdsInfo { num_counters : next_counter_id . as_u32 () , phys_counter_for_node , term_for_bcb : node_counters , expressions , }) }
}

macro_rules! all_coverage_in_mir_body_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function all_coverage_in_mir_body in module {}", module_path!());
    };
}

mkfn!{
    all_coverage_in_mir_body_introspect!();
    fn all_coverage_in_mir_body < 'a , 'tcx > (body : & 'a Body < 'tcx > ,) -> impl Iterator < Item = & 'a CoverageKind > { body . basic_blocks . iter () . flat_map (| bb_data | & bb_data . statements) . filter_map (| statement | { match statement . kind { StatementKind :: Coverage (ref kind) if ! is_inlined (body , statement) => Some (kind) , _ => None , } }) }
}

macro_rules! is_inlined_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_inlined in module {}", module_path!());
    };
}

mkfn!{
    is_inlined_introspect!();
    fn is_inlined (body : & Body < '_ > , statement : & Statement < '_ >) -> bool { let scope_data = & body . source_scopes [statement . source_info . scope] ; scope_data . inlined . is_some () || scope_data . inlined_parent_scope . is_some () }
}