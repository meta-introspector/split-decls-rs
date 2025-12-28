macro_rules! deps {
    () => {
        PoloniusFacts!();
        PoloniusLocationTable!();
    };
}

macro_rules! emit_move_facts {
    () => {
        deps!();
        # [doc = " Emit facts needed for move/init analysis: moves and assignments."] fn emit_move_facts (facts : & mut PoloniusFacts , body : & Body < '_ > , location_table : & PoloniusLocationTable , move_data : & MoveData < '_ > ,) { facts . path_is_var . extend (move_data . rev_lookup . iter_locals_enumerated () . map (| (l , r) | (r , l))) ; for (child , move_path) in move_data . move_paths . iter_enumerated () { if let Some (parent) = move_path . parent { facts . child_path . push ((child , parent)) ; } } let fn_entry_start = location_table . start_index (Location { block : START_BLOCK , statement_index : 0 }) ; for init in move_data . inits . iter () { match init . location { InitLocation :: Statement (location) => { let block_data = & body [location . block] ; let is_terminator = location . statement_index == block_data . statements . len () ; if is_terminator && init . kind == InitKind :: NonPanicPathOnly { for successor in block_data . terminator () . successors () { if body [successor] . is_cleanup { continue ; } let first_statement = Location { block : successor , statement_index : 0 } ; facts . path_assigned_at_base . push ((init . path , location_table . start_index (first_statement))) ; } } else { facts . path_assigned_at_base . push ((init . path , location_table . mid_index (location))) ; } } InitLocation :: Argument (local) => { assert ! (body . local_kind (local) == LocalKind :: Arg) ; facts . path_assigned_at_base . push ((init . path , fn_entry_start)) ; } } } for (local , path) in move_data . rev_lookup . iter_locals_enumerated () { if body . local_kind (local) != LocalKind :: Arg { facts . path_moved_at_base . push ((path , fn_entry_start)) ; } } facts . path_moved_at_base . extend (move_data . moves . iter () . map (| mo | (mo . path , location_table . mid_index (mo . source)))) ; }
    };
}

emit_move_facts!()