macro_rules! deps {
    () => {
        Mark!();
        Algorithm!();
        Error!();
        Negotiator!();
        Ancestors!();
        Graph!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Negotiator for Algorithm { fn known_common (& mut self , id : ObjectId , graph : & mut crate :: Graph < '_ , '_ >) -> Result < () , Error > { if graph . get (& id) . is_none_or (| commit | ! commit . data . flags . contains (Flags :: SEEN)) { self . add_to_queue (id , Flags :: COMMON_REF | Flags :: SEEN , graph) ? ; self . mark_common (id , Mark :: AncestorsOnly , Ancestors :: DirectUnseen , graph) ? ; } Ok (()) } fn add_tip (& mut self , id : ObjectId , graph : & mut crate :: Graph < '_ , '_ >) -> Result < () , Error > { self . add_to_queue (id , Flags :: SEEN , graph) } fn next_have (& mut self , graph : & mut crate :: Graph < '_ , '_ >) -> Option < Result < ObjectId , Error > > { loop { let id = self . revs . pop_value () . filter (| _ | self . non_common_revs != 0) ? ; let commit = graph . get_mut (& id) . expect ("it was added to the graph by now") ; let flags = & mut commit . data . flags ; * flags |= Flags :: POPPED ; if ! flags . contains (Flags :: COMMON) { self . non_common_revs -= 1 ; } let (res , mark) = if flags . contains (Flags :: COMMON) { (None , Flags :: COMMON | Flags :: SEEN) } else if flags . contains (Flags :: COMMON_REF) { (Some (id) , Flags :: COMMON | Flags :: SEEN) } else { (Some (id) , Flags :: SEEN) } ; for parent_id in commit . parents . clone () { if graph . get (& parent_id) . is_none_or (| commit | ! commit . data . flags . contains (Flags :: SEEN)) { if let Err (err) = self . add_to_queue (parent_id , mark , graph) { return Some (Err (err)) ; } } if mark . contains (Flags :: COMMON) { if let Err (err) = self . mark_common (parent_id , Mark :: AncestorsOnly , Ancestors :: AllUnseen , graph) { return Some (Err (err)) ; } } } if let Some (id) = res { return Some (Ok (id)) ; } } } fn in_common_with_remote (& mut self , id : ObjectId , graph : & mut crate :: Graph < '_ , '_ >) -> Result < bool , Error > { let known_to_be_common = graph . get (& id) . is_some_and (| commit | commit . data . flags . contains (Flags :: COMMON)) ; self . mark_common (id , Mark :: ThisCommitAndAncestors , Ancestors :: DirectUnseen , graph) ? ; Ok (known_to_be_common) } }
    };
}

impl_3!();