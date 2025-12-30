// Generated macro for draw_captions (function)
macro_rules! Depcrate_plots_stream_sparksdraw_captions {
() => {
// Module: crate::plots::stream_sparks
// Provides: {"draw_captions"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] fn draw_captions < DB : DrawingBackend > (abs_dl_root : & DrawingArea < DB , plotters :: coord :: Shift > , rel_dl_root : & DrawingArea < DB , plotters :: coord :: Shift > , abs_ul_root : & DrawingArea < DB , plotters :: coord :: Shift > , rel_ul_root : & DrawingArea < DB , plotters :: coord :: Shift > , params : & SparkPlotsParams , ds : & Datastore , stream_id : u64 , upload_data : bool , upper_x : u32 , dl_upper_y : u32 , ul_upper_y : u32 ,) { if let Some (captions) = SparkCaption :: from_data_store (ds , stream_id) { let x_offset = if params . captions_on_top { 0 } else if ds . application_proto == ApplicationProto :: Http2 { params . spark_dimension_x + 30 } else { params . spark_dimension_x } ; captions . draw (& params . colors . caption , abs_dl_root , (upper_x + x_offset) as i32 , dl_upper_y as i32 ,) ; captions . draw (& params . colors . caption , rel_dl_root , (upper_x + x_offset) as i32 , dl_upper_y as i32 ,) ; if upload_data { captions . draw (& params . colors . caption , abs_ul_root , (upper_x + x_offset) as i32 , ul_upper_y as i32 ,) ; captions . draw (& params . colors . caption , rel_ul_root , (upper_x + x_offset) as i32 , ul_upper_y as i32 ,) ; } } }
};
}
