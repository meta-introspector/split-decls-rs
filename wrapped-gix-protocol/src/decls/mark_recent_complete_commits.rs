macro_rules! deps {
    () => {
        Error!();
        Queue!();
    };
}

macro_rules! mark_recent_complete_commits {
    () => {
        deps!();
        # [doc = " Remove all commits that are more recent than the cut-off, which is the commit time of the oldest common commit we have with the server."] fn mark_recent_complete_commits (queue : & mut Queue , graph : & mut gix_negotiate :: Graph < '_ , '_ > , cutoff : SecondsSinceUnixEpoch ,) -> Result < () , Error > { let _span = gix_trace :: detail ! ("mark_recent_complete" , queue_len = queue . len ()) ; while let Some (id) = queue . peek () . and_then (| (commit_time , id) | (commit_time >= & cutoff) . then_some (* id)) { queue . pop_value () ; let commit = graph . get (& id) . expect ("definitely set when adding tips or parents") ; for parent_id in commit . parents . clone () { let mut was_complete = false ; if let Some (parent) = graph . get_or_insert_commit (parent_id , | md | { was_complete = md . flags . contains (Flags :: COMPLETE) ; md . flags |= Flags :: COMPLETE ; }) ? . filter (| _ | ! was_complete) { queue . insert (parent . commit_time , parent_id) ; } } } Ok (()) }
    };
}

mark_recent_complete_commits!();