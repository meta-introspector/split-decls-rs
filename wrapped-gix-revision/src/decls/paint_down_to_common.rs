macro_rules! deps {
    () => {
        Flags!();
        Error!();
        GenThenTime!();
    };
}

macro_rules! paint_down_to_common {
    () => {
        deps!();
        fn paint_down_to_common (first : ObjectId , others : & [ObjectId] , graph : & mut Graph < '_ , '_ , graph :: Commit < Flags > > ,) -> Result < Vec < (ObjectId , GenThenTime) > , Error > { let mut queue = PriorityQueue :: < GenThenTime , ObjectId > :: new () ; graph . get_or_insert_full_commit (first , | commit | { commit . data |= Flags :: COMMIT1 ; queue . insert (GenThenTime :: from (& * commit) , first) ; }) ? ; for other in others { graph . get_or_insert_full_commit (* other , | commit | { commit . data |= Flags :: COMMIT2 ; queue . insert (GenThenTime :: from (& * commit) , * other) ; }) ? ; } let mut out = Vec :: new () ; while queue . iter_unordered () . any (| id | graph . get (id) . is_some_and (| commit | ! commit . data . contains (Flags :: STALE))) { let (info , commit_id) = queue . pop () . expect ("we have non-stale") ; let commit = graph . get_mut (& commit_id) . expect ("everything queued is in graph") ; let mut flags_without_result = commit . data & (Flags :: COMMIT1 | Flags :: COMMIT2 | Flags :: STALE) ; if flags_without_result == (Flags :: COMMIT1 | Flags :: COMMIT2) { if ! commit . data . contains (Flags :: RESULT) { commit . data |= Flags :: RESULT ; out . push ((commit_id , info)) ; } flags_without_result |= Flags :: STALE ; } for parent_id in commit . parents . clone () { graph . get_or_insert_full_commit (parent_id , | parent | { if (parent . data & flags_without_result) != flags_without_result { parent . data |= flags_without_result ; queue . insert (GenThenTime :: from (& * parent) , parent_id) ; } }) ? ; } } Ok (out) }
    };
}

paint_down_to_common!();