macro_rules! deps {
    () => {
        Error!();
        Graph!();
        Algorithm!();
        Negotiator!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Negotiator for Algorithm { fn known_common (& mut self , id : ObjectId , graph : & mut crate :: Graph < '_ , '_ >) -> Result < () , Error > { if graph . get (& id) . is_some_and (| commit | commit . data . flags . contains (Flags :: SEEN)) { return Ok (()) ; } self . add_to_queue (id , Flags :: ADVERTISED , graph) } fn add_tip (& mut self , id : ObjectId , graph : & mut crate :: Graph < '_ , '_ >) -> Result < () , Error > { if graph . get (& id) . is_some_and (| commit | commit . data . flags . contains (Flags :: SEEN)) { return Ok (()) ; } self . add_to_queue (id , Flags :: default () , graph) } fn next_have (& mut self , graph : & mut crate :: Graph < '_ , '_ >) -> Option < Result < ObjectId , Error > > { loop { let id = self . revs . pop_value () . filter (| _ | self . non_common_revs != 0) ? ; let commit = graph . get_mut (& id) . expect ("it was added to the graph by now") ; commit . data . flags |= Flags :: POPPED ; if ! commit . data . flags . contains (Flags :: COMMON) { self . non_common_revs -= 1 ; } let mut to_send = None ; if ! commit . data . flags . contains (Flags :: COMMON) && commit . data . ttl == 0 { to_send = Some (id) ; } let data = commit . data ; let mut parent_pushed = false ; for parent_id in commit . parents . clone () { parent_pushed |= match self . push_parent (data , parent_id , graph) { Ok (r) => r , Err (err) => return Some (Err (err)) , } } if ! data . flags . contains (Flags :: COMMON) && ! parent_pushed { to_send = Some (id) ; } if let Some (to_send) = to_send { return Some (Ok (to_send)) ; } } } fn in_common_with_remote (& mut self , id : ObjectId , graph : & mut crate :: Graph < '_ , '_ >) -> Result < bool , Error > { let mut was_seen = false ; let known_to_be_common = graph . get (& id) . is_some_and (| commit | { was_seen = commit . data . flags . contains (Flags :: SEEN) ; commit . data . flags . contains (Flags :: COMMON) }) ; assert ! (was_seen , "Cannot receive ACK for commit we didn't send a HAVE for: {id}") ; self . mark_common (id , graph) ? ; Ok (known_to_be_common) } }
    };
}

impl_13!()