macro_rules! deps {
    () => {
        IteratorType!();
        Relation!();
        ChangeId!();
        Error!();
        TreeInfoTuple!();
        Visit!();
    };
}

macro_rules! catchup_lhs_with_rhs {
    () => {
        deps!();
        fn catchup_lhs_with_rhs (lhs_entries : & mut IteratorType < TreeRefIter < '_ > > , lhs : EntryRef < '_ > , rhs : EntryRef < '_ > , queue : & mut VecDeque < TreeInfoTuple > , change_id : & mut ChangeId , relation_to_propagate : Option < Relation > , delegate : & mut impl Visit ,) -> Result < () , Error > { use std :: cmp :: Ordering :: * ; delete_entry_schedule_recursion (lhs , queue , change_id , relation_to_propagate , delegate) ? ; loop { match lhs_entries . peek () { Some (Ok (lhs)) => match compare (lhs , & rhs) { Equal => { let lhs = lhs_entries . next () . expect ("the peeked item to be present") ? ; delegate . pop_path_component () ; handle_lhs_and_rhs_with_equal_filenames (lhs , rhs , queue , change_id , relation_to_propagate , delegate ,) ? ; break ; } Less => { let lhs = lhs_entries . next () . expect ("the peeked item to be present") ? ; delegate . pop_path_component () ; delete_entry_schedule_recursion (lhs , queue , change_id , relation_to_propagate , delegate) ? ; } Greater => { delegate . pop_path_component () ; add_entry_schedule_recursion (rhs , queue , change_id , relation_to_propagate , delegate) ? ; break ; } } , Some (Err (err)) => return Err (Error :: EntriesDecode (err . to_owned ())) , None => { delegate . pop_path_component () ; add_entry_schedule_recursion (rhs , queue , change_id , relation_to_propagate , delegate) ? ; break ; } } } Ok (()) }
    };
}

catchup_lhs_with_rhs!()