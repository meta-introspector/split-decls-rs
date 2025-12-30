// Generated macro for impl_926 (impl)
macro_rules! Depcrate_vtab_seriesimpl_926 {
() => {
// Module: crate::vtab::series
// Provides: {"impl_926"}
// Dependencies: {}
impl SeriesTabCursor < '_ > { fn new < 'vtab > () -> SeriesTabCursor < 'vtab > { SeriesTabCursor { base : ffi :: sqlite3_vtab_cursor :: default () , is_desc : false , row_id : 0 , value : 0 , min_value : 0 , max_value : 0 , step : 0 , phantom : PhantomData , } } }
};
}
