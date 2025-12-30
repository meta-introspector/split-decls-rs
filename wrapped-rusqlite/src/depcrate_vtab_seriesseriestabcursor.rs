// Generated macro for SeriesTabCursor (struct)
macro_rules! Depcrate_vtab_seriesSeriesTabCursor {
() => {
// Module: crate::vtab::series
// Provides: {"SeriesTabCursor"}
// Dependencies: {}
# [doc = " A cursor for the Series virtual table"] # [repr (C)] struct SeriesTabCursor < 'vtab > { # [doc = " Base class. Must be first"] base : ffi :: sqlite3_vtab_cursor , # [doc = " True to count down rather than up"] is_desc : bool , # [doc = " The rowid"] row_id : i64 , # [doc = " Current value (\"value\")"] value : i64 , # [doc = " Minimum value (\"start\")"] min_value : i64 , # [doc = " Maximum value (\"stop\")"] max_value : i64 , # [doc = " Increment (\"step\")"] step : i64 , phantom : PhantomData < & 'vtab SeriesTab > , }
};
}
