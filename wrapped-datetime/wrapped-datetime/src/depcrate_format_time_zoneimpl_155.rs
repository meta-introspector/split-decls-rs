// Generated macro for impl_155 (impl)
macro_rules! Depcrate_format_time_zoneimpl_155 {
() => {
// Module: crate::format::time_zone
// Provides: {"impl_155"}
// Dependencies: {}
impl Iso8601Format { pub (crate) fn with_z (length : FieldLength) -> Self { match length { FieldLength :: One => Self { extended : false , z : true , minutes : Minutes :: Optional , seconds : Seconds :: Never , } , FieldLength :: Two => Self { extended : false , z : true , minutes : Minutes :: Required , seconds : Seconds :: Never , } , FieldLength :: Three => Self { extended : true , z : true , minutes : Minutes :: Required , seconds : Seconds :: Never , } , FieldLength :: Four => Self { extended : false , z : true , minutes : Minutes :: Required , seconds : Seconds :: Optional , } , _ => Self { extended : true , z : true , minutes : Minutes :: Required , seconds : Seconds :: Optional , } , } } pub (crate) fn without_z (length : FieldLength) -> Self { match length { FieldLength :: One => Self { extended : false , z : false , minutes : Minutes :: Optional , seconds : Seconds :: Never , } , FieldLength :: Two => Self { extended : false , z : false , minutes : Minutes :: Required , seconds : Seconds :: Never , } , FieldLength :: Three => Self { extended : true , z : false , minutes : Minutes :: Required , seconds : Seconds :: Never , } , FieldLength :: Four => Self { extended : false , z : false , minutes : Minutes :: Required , seconds : Seconds :: Optional , } , _ => Self { extended : true , z : false , minutes : Minutes :: Required , seconds : Seconds :: Optional , } , } } }
};
}
