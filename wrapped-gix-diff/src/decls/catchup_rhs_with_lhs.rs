macro_rules! deps {
    () => {
        TreeInfoTuple!();
        ChangeId!();
        IteratorType!();
        Relation!();
        Error!();
        Visit!();
    };
}

macro_rules! catchup_rhs_with_lhs {
    () => {
        deps!();
        fn catchup_rhs_with_lhs (rhs_entries : & mut IteratorType < TreeRefIter < '_ > > , lhs : EntryRef < '_ > , rhs : EntryRef < '_ > , queue : & mut VecDeque < TreeInfoTuple > , change_id : & mut ChangeId , relation_to_propagate : Option < Relation > , delegate : & mut impl Visit ,) -> Result < () , Error > { use std :: cmp :: Ordering :: * ; add_entry_schedule_recursion (rhs , queue , change_id , relation_to_propagate , delegate) ? ; loop { match rhs_entries . peek () { Some (Ok (rhs)) => match compare (& lhs , rhs) { Equal => { let rhs = rhs_entries . next () . transpose () ? . expect ("the peeked item to be present") ; delegate . pop_path_component () ; handle_lhs_and_rhs_with_equal_filenames (lhs , rhs , queue , change_id , relation_to_propagate , delegate ,) ? ; break ; } Greater => { let rhs = rhs_entries . next () . transpose () ? . expect ("the peeked item to be present") ; delegate . pop_path_component () ; add_entry_schedule_recursion (rhs , queue , change_id , relation_to_propagate , delegate) ? ; } Less => { delegate . pop_path_component () ; delete_entry_schedule_recursion (lhs , queue , change_id , relation_to_propagate , delegate) ? ; break ; } } , Some (Err (err)) => return Err (Error :: EntriesDecode (err . to_owned ())) , None => { delegate . pop_path_component () ; delete_entry_schedule_recursion (lhs , queue , change_id , relation_to_propagate , delegate) ? ; break ; } } } Ok (()) }
    };
}

catchup_rhs_with_lhs!();