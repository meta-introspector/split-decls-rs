// Generated macro for format_rest (function)
macro_rules! Depcrateformat_rest {
() => {
// Module: crate
// Provides: {"format_rest"}
// Dependencies: {}
fn format_rest (rest : Vec < (i64 , FixedTimespan) >) -> String { let mut ret = "&[\n" . to_string () ; for (start , FixedTimespan { utc_offset , dst_offset , name , } ,) in rest { ret . push_str (& format ! ("                ({start}, FixedTimespan {{ \
             offset: {offset}, name: {name:?} \
             }}),\n" , offset = utc_offset + dst_offset ,)) ; } ret . push_str ("            ]") ; ret }
};
}
