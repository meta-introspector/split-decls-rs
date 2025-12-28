macro_rules! deps {
    () => {
        RefLogMessage!();
        WritePackedRefs!();
        DryRun!();
        Connection!();
    };
}

macro_rules! Prepare {
    () => {
        deps!();
        # [doc = " A structure to hold the result of the handshake with the remote and configure the upcoming fetch operation."] pub struct Prepare < 'remote , 'repo , T > where T : Transport , { con : Option < Connection < 'remote , 'repo , T > > , ref_map : RefMap , dry_run : DryRun , reflog_message : Option < RefLogMessage > , write_packed_refs : WritePackedRefs , shallow : remote :: fetch :: Shallow , }
    };
}

Prepare!();