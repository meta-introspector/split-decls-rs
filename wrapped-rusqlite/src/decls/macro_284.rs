macro_rules! macro_284 {
    () => {
        bitflags :: bitflags ! { # [doc = " Trace event codes"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] # [non_exhaustive] # [repr (C)] pub struct TraceEventCodes : c_uint { # [doc = " when a prepared statement first begins running and possibly at other times during the execution"] # [doc = " of the prepared statement, such as at the start of each trigger subprogram"] const SQLITE_TRACE_STMT = ffi :: SQLITE_TRACE_STMT ; # [doc = " when the statement finishes"] const SQLITE_TRACE_PROFILE = ffi :: SQLITE_TRACE_PROFILE ; # [doc = " whenever a prepared statement generates a single row of result"] const SQLITE_TRACE_ROW = ffi :: SQLITE_TRACE_ROW ; # [doc = " when a database connection closes"] const SQLITE_TRACE_CLOSE = ffi :: SQLITE_TRACE_CLOSE ; } }
    };
}

macro_284!()