macro_rules! deps {
    () => {
        SeriesTabCursor!();
    };
}

macro_rules! impl_639 {
    () => {
        deps!();
        impl SeriesTabCursor < '_ > { fn new < 'vtab > () -> SeriesTabCursor < 'vtab > { SeriesTabCursor { base : ffi :: sqlite3_vtab_cursor :: default () , is_desc : false , row_id : 0 , value : 0 , min_value : 0 , max_value : 0 , step : 0 , phantom : PhantomData , } } }
    };
}

impl_639!();