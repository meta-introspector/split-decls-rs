// Generated macro for impl_95 (impl)
macro_rules! Depcrate_errorimpl_95 {
() => {
// Module: crate::error
// Provides: {"impl_95"}
// Dependencies: {}
impl From < proto :: Error > for Error { fn from (src : proto :: Error) -> Error { use crate :: proto :: Error :: * ; Error { kind : match src { Reset (stream_id , reason , initiator) => Kind :: Reset (stream_id , reason , initiator) , GoAway (debug_data , reason , initiator) => { Kind :: GoAway (debug_data , reason , initiator) } Io (kind , inner) => { Kind :: Io (inner . map_or_else (| | kind . into () , | inner | io :: Error :: new (kind , inner))) } } , } } }
};
}
