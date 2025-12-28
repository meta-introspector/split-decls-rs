macro_rules! deps {
    () => {
        Error!();
        Metadata!();
        Algorithm!();
        Graph!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Algorithm { # [doc = " Add `id` to our priority queue and *add* `flags` to it."] fn add_to_queue (& mut self , id : ObjectId , mark : Flags , graph : & mut crate :: Graph < '_ , '_ >) -> Result < () , Error > { let commit = graph . get_or_insert_commit (id , | entry | { entry . flags |= mark | Flags :: SEEN ; }) ? ; if let Some (timestamp) = commit . map (| c | c . commit_time) { self . revs . insert (timestamp , id) ; if ! mark . contains (Flags :: COMMON) { self . non_common_revs += 1 ; } } Ok (()) } fn mark_common (& mut self , id : ObjectId , graph : & mut crate :: Graph < '_ , '_ >) -> Result < () , Error > { let mut is_common = false ; if let Some (commit) = graph . get_or_insert_commit (id , | entry | { is_common = entry . flags . contains (Flags :: COMMON) ; entry . flags |= Flags :: COMMON ; }) ? . filter (| _ | ! is_common) { let mut queue = gix_revwalk :: PriorityQueue :: from_iter (Some ((commit . commit_time , id))) ; while let Some (id) = queue . pop_value () { if let Some (commit) = graph . get_or_insert_commit (id , | entry | { if ! entry . flags . contains (Flags :: POPPED) { self . non_common_revs -= 1 ; } }) ? { for parent_id in commit . parents . clone () { if ! graph . contains (& parent_id) { continue ; } let mut was_unseen_or_common = false ; if let Some (parent) = graph . get_or_insert_commit (parent_id , | entry | { was_unseen_or_common = ! entry . flags . contains (Flags :: SEEN) || entry . flags . contains (Flags :: COMMON) ; entry . flags |= Flags :: COMMON ; }) ? . filter (| _ | ! was_unseen_or_common) { queue . insert (parent . commit_time , parent_id) ; } } } } } Ok (()) } fn push_parent (& mut self , entry : Metadata , parent_id : ObjectId , graph : & mut crate :: Graph < '_ , '_ > ,) -> Result < bool , Error > { let mut was_seen = false ; if let Some (parent) = graph . get (& parent_id) . inspect (| parent | { was_seen = parent . data . flags . contains (Flags :: SEEN) ; }) . filter (| _ | was_seen) { if parent . data . flags . contains (Flags :: POPPED) { return Ok (false) ; } } else { self . add_to_queue (parent_id , Flags :: default () , graph) ? ; } if entry . flags . intersects (Flags :: COMMON | Flags :: ADVERTISED) { self . mark_common (parent_id , graph) ? ; } else { let new_original_ttl = if entry . ttl > 0 { entry . original_ttl } else { entry . original_ttl * 3 / 2 + 1 } ; let new_ttl = if entry . ttl > 0 { entry . ttl - 1 } else { new_original_ttl } ; let parent = graph . get_mut (& parent_id) . expect ("present or inserted") ; if parent . data . original_ttl < new_original_ttl { parent . data . original_ttl = new_original_ttl ; parent . data . ttl = new_ttl ; } } Ok (true) } }
    };
}

impl_12!();