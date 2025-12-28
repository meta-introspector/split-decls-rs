macro_rules! deps {
    () => {
        Relation!();
        Visit!();
        Error!();
        ChangeId!();
        Change!();
        TreeInfoTuple!();
    };
}

macro_rules! add_entry_schedule_recursion {
    () => {
        deps!();
        fn add_entry_schedule_recursion (entry : EntryRef < '_ > , queue : & mut VecDeque < TreeInfoTuple > , change_id : & mut ChangeId , relation_to_propagate : Option < Relation > , delegate : & mut impl Visit ,) -> Result < () , Error > { delegate . push_path_component (entry . filename) ; let relation = relation_to_propagate . or_else (| | { entry . mode . is_tree () . then (| | { * change_id += 1 ; Relation :: Parent (* change_id) }) }) ; if delegate . visit (Change :: Addition { entry_mode : entry . mode , oid : entry . oid . to_owned () , relation , }) . cancelled () { return Err (Error :: Cancelled) ; } if entry . mode . is_tree () { delegate . pop_path_component () ; delegate . push_back_tracked_path_component (entry . filename) ; queue . push_back ((None , Some (entry . oid . to_owned ()) , to_child (relation))) ; } Ok (()) }
    };
}

add_entry_schedule_recursion!();