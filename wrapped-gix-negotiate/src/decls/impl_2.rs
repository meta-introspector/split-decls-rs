macro_rules! deps {
    () => {
        Ancestors!();
        Algorithm!();
        Error!();
        Mark!();
        Graph!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Algorithm { # [doc = " Add `id` to our priority queue and *add* `flags` to it."] fn add_to_queue (& mut self , id : ObjectId , mark : Flags , graph : & mut crate :: Graph < '_ , '_ >) -> Result < () , Error > { let mut is_common = false ; let mut has_mark = false ; if let Some (commit) = graph . get_or_insert_commit (id , | data | { has_mark = data . flags . intersects (mark) ; data . flags |= mark ; is_common = data . flags . contains (Flags :: COMMON) ; }) ? . filter (| _ | ! has_mark) { self . revs . insert (commit . commit_time , id) ; if ! is_common { self . non_common_revs += 1 ; } } Ok (()) } fn mark_common (& mut self , id : ObjectId , mode : Mark , ancestors : Ancestors , graph : & mut crate :: Graph < '_ , '_ > ,) -> Result < () , Error > { let mut is_common = false ; if let Some (commit) = graph . get_or_insert_commit (id , | data | is_common = data . flags . contains (Flags :: COMMON)) ? . filter (| _ | ! is_common) { let mut queue = gix_revwalk :: PriorityQueue :: from_iter (Some ((commit . commit_time , (id , 0_usize)))) ; if let Mark :: ThisCommitAndAncestors = mode { commit . data . flags |= Flags :: COMMON ; if commit . data . flags . contains (Flags :: SEEN) && ! commit . data . flags . contains (Flags :: POPPED) { self . non_common_revs -= 1 ; } } while let Some ((id , generation)) = queue . pop_value () { if graph . get (& id) . is_none_or (| commit | ! commit . data . flags . contains (Flags :: SEEN)) { self . add_to_queue (id , Flags :: SEEN , graph) ? ; } else if matches ! (ancestors , Ancestors :: AllUnseen) || generation < 2 { if let Some (commit) = graph . get_or_insert_commit (id , | _ | { }) ? { for parent_id in commit . parents . clone () { let mut prev_flags = Flags :: default () ; if let Some (parent) = graph . get_or_insert_commit (parent_id , | data | { prev_flags = data . flags ; data . flags |= Flags :: COMMON ; }) ? . filter (| _ | ! prev_flags . contains (Flags :: COMMON)) { if prev_flags . contains (Flags :: SEEN) && ! prev_flags . contains (Flags :: POPPED) { self . non_common_revs -= 1 ; } queue . insert (parent . commit_time , (parent_id , generation + 1)) ; } } } } } } Ok (()) } }
    };
}

impl_2!();