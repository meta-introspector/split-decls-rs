// Generated macro for poll_stream (macro)
macro_rules! Depcrate_stream_merge_tuplepoll_stream {
() => {
// Module: crate::stream::merge::tuple
// Provides: {"poll_stream"}
// Dependencies: {}
macro_rules ! poll_stream { ($ stream_idx : tt , $ iteration : ident , $ this : ident , $ streams : ident . $ stream_member : ident , $ cx : ident , $ len_streams : ident) => { if $ stream_idx == $ iteration { match unsafe { Pin :: new_unchecked (& mut $ streams .$ stream_member) } . poll_next (& mut $ cx) { Poll :: Ready (Some (item)) => { $ this . wakers . readiness () . set_ready ($ stream_idx) ; return Poll :: Ready (Some (item)) ; } Poll :: Ready (None) => { *$ this . completed += 1 ; $ this . state [$ stream_idx] . set_none () ; if *$ this . completed == $ len_streams { return Poll :: Ready (None) ; } } Poll :: Pending => { } } } } ; }
};
}
