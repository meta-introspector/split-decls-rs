// Generated macro for from_qlog_stream_type_set (function)
macro_rules! Depcrate_recordreplay_qlogfrom_qlog_stream_type_set {
() => {
// Module: crate::recordreplay::qlog
// Provides: {"from_qlog_stream_type_set"}
// Dependencies: {}
fn from_qlog_stream_type_set (st : & H3StreamTypeSet , ex_data : & ExData ,) -> Vec < Action > { let mut actions = vec ! [] ; let fin_stream = parse_ex_data (ex_data) ; let stream_type = match st . stream_type { qlog :: events :: h3 :: H3StreamType :: Control => Some (0x0) , qlog :: events :: h3 :: H3StreamType :: Push => Some (0x1) , qlog :: events :: h3 :: H3StreamType :: QpackEncode => Some (0x2) , qlog :: events :: h3 :: H3StreamType :: QpackDecode => Some (0x3) , qlog :: events :: h3 :: H3StreamType :: Reserved | qlog :: events :: h3 :: H3StreamType :: Unknown => st . stream_type_value , _ => None , } ; if let Some (ty) = stream_type { actions . push (Action :: OpenUniStream { stream_id : st . stream_id , fin_stream , stream_type : ty , }) } actions }
};
}
