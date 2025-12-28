macro_rules! deps {
    () => {
        Relation!();
        Error!();
        Visit!();
        ChangeId!();
        TreeInfoTuple!();
        Change!();
    };
}

macro_rules! delete_entry_schedule_recursion {
    () => {
        deps!();
        fn delete_entry_schedule_recursion (entry : EntryRef < '_ > , queue : & mut VecDeque < TreeInfoTuple > , change_id : & mut ChangeId , relation_to_propagate : Option < Relation > , delegate : & mut impl Visit ,) -> Result < () , Error > { delegate . push_path_component (entry . filename) ; let relation = relation_to_propagate . or_else (| | { entry . mode . is_tree () . then (| | { * change_id += 1 ; Relation :: Parent (* change_id) }) }) ; let is_cancelled = delegate . visit (Change :: Deletion { entry_mode : entry . mode , oid : entry . oid . to_owned () , relation , }) . cancelled () ; if is_cancelled { return Err (Error :: Cancelled) ; } if entry . mode . is_tree () { delegate . pop_path_component () ; delegate . push_back_tracked_path_component (entry . filename) ; queue . push_back ((Some (entry . oid . to_owned ()) , None , to_child (relation))) ; } Ok (()) }
    };
}

delete_entry_schedule_recursion!();