mkuse!{use std :: iter ;}
mkuse!{use either :: Either ;}
mkuse!{use rustc_middle :: mir :: { Body , Local , LocalKind , Location , START_BLOCK } ;}
mkuse!{use rustc_middle :: ty :: { GenericArg , TyCtxt } ;}
mkuse!{use rustc_mir_dataflow :: move_paths :: { InitKind , InitLocation , MoveData } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: borrow_set :: BorrowSet ;}
mkuse!{use crate :: constraints :: OutlivesConstraint ;}
mkuse!{use crate :: handle_placeholders :: LoweredConstraints ;}
mkuse!{use crate :: type_check :: free_region_relations :: UniversalRegionRelations ;}
mkuse!{use crate :: universal_regions :: UniversalRegions ;}
mkmod!{accesses, { 
                getname!(accesses);
                getsrc!(accesses);
                getpath!(accesses);
                get_deps!(accesses);
                get_crates!(accesses);
                mkinclude!(accesses);
                 
            }}
mkmod!{loan_invalidations, { 
                getname!(loan_invalidations);
                getsrc!(loan_invalidations);
                getpath!(loan_invalidations);
                get_deps!(loan_invalidations);
                get_crates!(loan_invalidations);
                mkinclude!(loan_invalidations);
                 
            }}
mkmod!{loan_kills, { 
                getname!(loan_kills);
                getsrc!(loan_kills);
                getpath!(loan_kills);
                get_deps!(loan_kills);
                get_crates!(loan_kills);
                mkinclude!(loan_kills);
                 
            }}
mkmod!{location, { 
                getname!(location);
                getsrc!(location);
                getpath!(location);
                get_deps!(location);
                get_crates!(location);
                mkinclude!(location);
                 
            }}
mkuse!{pub use self :: location :: * ;}
mkmod!{facts, { 
                getname!(facts);
                getsrc!(facts);
                getpath!(facts);
                get_deps!(facts);
                get_crates!(facts);
                mkinclude!(facts);
                 
            }}
mkuse!{pub use self :: facts :: * ;}

macro_rules! emit_facts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_facts in module {}", module_path!());
    };
}

mkfn!{
    emit_facts_introspect!();
    # [doc = " When requested, emit most of the facts needed by polonius:"] # [doc = " - moves and assignments"] # [doc = " - universal regions and their relations"] # [doc = " - CFG points and edges"] # [doc = " - loan kills"] # [doc = " - loan invalidations"] # [doc = " - access facts such as variable definitions, uses, drops, and path accesses"] # [doc = " - outlives constraints"] # [doc = ""] # [doc = " The rest of the facts are emitted during typeck and liveness."] pub (crate) fn emit_facts < 'tcx > (facts : & mut Option < PoloniusFacts > , tcx : TyCtxt < 'tcx > , location_table : & PoloniusLocationTable , body : & Body < 'tcx > , borrow_set : & BorrowSet < 'tcx > , move_data : & MoveData < 'tcx > , universal_region_relations : & UniversalRegionRelations < 'tcx > , constraints : & LoweredConstraints < 'tcx > ,) { let Some (facts) = facts else { return ; } ; let _prof_timer = tcx . prof . generic_activity ("polonius_fact_generation") ; emit_move_facts (facts , body , location_table , move_data) ; emit_universal_region_facts (facts , borrow_set , universal_region_relations) ; loan_kills :: emit_loan_kills (tcx , facts , body , location_table , borrow_set) ; loan_invalidations :: emit_loan_invalidations (tcx , facts , body , location_table , borrow_set) ; accesses :: emit_access_facts (tcx , facts , body , location_table , move_data , & universal_region_relations . universal_regions ,) ; emit_outlives_facts (facts , location_table , constraints) ; }
}

macro_rules! emit_move_facts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_move_facts in module {}", module_path!());
    };
}

mkfn!{
    emit_move_facts_introspect!();
    # [doc = " Emit facts needed for move/init analysis: moves and assignments."] fn emit_move_facts (facts : & mut PoloniusFacts , body : & Body < '_ > , location_table : & PoloniusLocationTable , move_data : & MoveData < '_ > ,) { facts . path_is_var . extend (move_data . rev_lookup . iter_locals_enumerated () . map (| (l , r) | (r , l))) ; for (child , move_path) in move_data . move_paths . iter_enumerated () { if let Some (parent) = move_path . parent { facts . child_path . push ((child , parent)) ; } } let fn_entry_start = location_table . start_index (Location { block : START_BLOCK , statement_index : 0 }) ; for init in move_data . inits . iter () { match init . location { InitLocation :: Statement (location) => { let block_data = & body [location . block] ; let is_terminator = location . statement_index == block_data . statements . len () ; if is_terminator && init . kind == InitKind :: NonPanicPathOnly { for successor in block_data . terminator () . successors () { if body [successor] . is_cleanup { continue ; } let first_statement = Location { block : successor , statement_index : 0 } ; facts . path_assigned_at_base . push ((init . path , location_table . start_index (first_statement))) ; } } else { facts . path_assigned_at_base . push ((init . path , location_table . mid_index (location))) ; } } InitLocation :: Argument (local) => { assert ! (body . local_kind (local) == LocalKind :: Arg) ; facts . path_assigned_at_base . push ((init . path , fn_entry_start)) ; } } } for (local , path) in move_data . rev_lookup . iter_locals_enumerated () { if body . local_kind (local) != LocalKind :: Arg { facts . path_moved_at_base . push ((path , fn_entry_start)) ; } } facts . path_moved_at_base . extend (move_data . moves . iter () . map (| mo | (mo . path , location_table . mid_index (mo . source)))) ; }
}

macro_rules! emit_universal_region_facts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_universal_region_facts in module {}", module_path!());
    };
}

mkfn!{
    emit_universal_region_facts_introspect!();
    # [doc = " Emit universal regions facts, and their relations."] fn emit_universal_region_facts (facts : & mut PoloniusFacts , borrow_set : & BorrowSet < '_ > , universal_region_relations : & UniversalRegionRelations < '_ > ,) { let universal_regions = & universal_region_relations . universal_regions ; facts . universal_region . extend (universal_regions . universal_regions_iter () . map (PoloniusRegionVid :: from)) ; let borrow_count = borrow_set . len () ; debug ! ("emit_universal_region_facts: polonius placeholders, num_universals={}, borrow_count={}" , universal_regions . len () , borrow_count) ; for universal_region in universal_regions . universal_regions_iter () { let universal_region_idx = universal_region . index () ; let placeholder_loan_idx = borrow_count + universal_region_idx ; facts . placeholder . push ((universal_region . into () , placeholder_loan_idx . into ())) ; } for (fr1 , fr2) in universal_region_relations . known_outlives () { if fr1 != fr2 { debug ! ("emit_universal_region_facts: emitting polonius `known_placeholder_subset` \
                     fr1={:?}, fr2={:?}" , fr1 , fr2) ; facts . known_placeholder_subset . push ((fr1 . into () , fr2 . into ())) ; } } }
}

macro_rules! emit_drop_facts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_drop_facts in module {}", module_path!());
    };
}

mkfn!{
    emit_drop_facts_introspect!();
    # [doc = " For every potentially drop()-touched region `region` in `local`'s type"] # [doc = " (`kind`), emit a `drop_of_var_derefs_origin(local, origin)` fact."] pub (crate) fn emit_drop_facts < 'tcx > (tcx : TyCtxt < 'tcx > , local : Local , kind : & GenericArg < 'tcx > , universal_regions : & UniversalRegions < 'tcx > , facts : & mut Option < PoloniusFacts > ,) { debug ! ("emit_drop_facts(local={:?}, kind={:?}" , local , kind) ; let Some (facts) = facts . as_mut () else { return } ; let _prof_timer = tcx . prof . generic_activity ("polonius_fact_generation") ; tcx . for_each_free_region (kind , | drop_live_region | { let region_vid = universal_regions . to_region_vid (drop_live_region) ; facts . drop_of_var_derefs_origin . push ((local , region_vid . into ())) ; }) ; }
}

macro_rules! emit_outlives_facts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_outlives_facts in module {}", module_path!());
    };
}

mkfn!{
    emit_outlives_facts_introspect!();
    # [doc = " Emit facts about the outlives constraints: the `subset` base relation, i.e. not a transitive"] # [doc = " closure."] fn emit_outlives_facts < 'tcx > (facts : & mut PoloniusFacts , location_table : & PoloniusLocationTable , constraints : & LoweredConstraints < 'tcx > ,) { facts . subset_base . extend (constraints . outlives_constraints . outlives () . iter () . flat_map (| constraint : & OutlivesConstraint < '_ > | { if let Some (from_location) = constraint . locations . from_location () { Either :: Left (iter :: once ((constraint . sup . into () , constraint . sub . into () , location_table . mid_index (from_location) ,))) } else { Either :: Right (location_table . all_points () . map (move | location | { (constraint . sup . into () , constraint . sub . into () , location) }) ,) } } ,)) ; }
}