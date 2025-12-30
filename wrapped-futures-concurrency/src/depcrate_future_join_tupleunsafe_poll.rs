// Generated macro for unsafe_poll (macro)
macro_rules! Depcrate_future_join_tupleunsafe_poll {
() => {
// Module: crate::future::join::tuple
// Provides: {"unsafe_poll"}
// Dependencies: {}
# [doc = " Generates the `poll` call for every `Future` inside `$futures`."] # [doc = ""] # [doc = " SAFETY: pretty please only call this after having made very sure that the future you're trying"] # [doc = " to call is actually marked `ready!`. If Rust had unsafe macros, this would be one."] macro_rules ! unsafe_poll { (@ inner $ iteration : ident , $ this : ident , $ futures : ident , $ cx : ident , $ fut_name : ident $ ($ F : ident) * | $ fut_idx : tt $ ($ rest : tt) *) => { if $ fut_idx == $ iteration { if let Poll :: Ready (value) = unsafe { $ futures .$ fut_name . as_mut () . map_unchecked_mut (| t | t . deref_mut ()) . poll (& mut $ cx) } { $ this . outputs .$ fut_idx . write (value) ; *$ this . completed += 1 ; $ this . state [$ fut_idx] . set_ready () ; unsafe { ManuallyDrop :: drop ($ futures .$ fut_name . as_mut () . get_unchecked_mut ()) } ; } } unsafe_poll ! (@ inner $ iteration , $ this , $ futures , $ cx , $ ($ F) * | $ ($ rest) *) ; } ; (@ inner $ iteration : ident , $ this : ident , $ futures : ident , $ cx : ident , | $ ($ rest : tt) *) => { } ; ($ iteration : ident , $ this : ident , $ futures : ident , $ cx : ident , $ LEN : ident , $ ($ F : ident ,) +) => { unsafe_poll ! (@ inner $ iteration , $ this , $ futures , $ cx , $ ($ F) + | 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14) ; } ; }
};
}
