macro_rules! deps {
    () => {
        Row!();
        ConnRef!();
        StmtRef!();
    };
}

macro_rules! TraceEvent {
    () => {
        deps!();
        # [doc = " Trace event"] # [non_exhaustive] pub enum TraceEvent < 's > { # [doc = " when a prepared statement first begins running and possibly at other times during the execution"] # [doc = " of the prepared statement, such as at the start of each trigger subprogram"] Stmt (StmtRef < 's > , & 's str) , # [doc = " when the statement finishes"] Profile (StmtRef < 's > , Duration) , # [doc = " whenever a prepared statement generates a single row of result"] Row (StmtRef < 's >) , # [doc = " when a database connection closes"] Close (ConnRef < 's >) , }
    };
}

TraceEvent!()