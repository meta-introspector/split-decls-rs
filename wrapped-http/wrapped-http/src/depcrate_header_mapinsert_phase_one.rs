// Generated macro for insert_phase_one (macro)
macro_rules! Depcrate_header_mapinsert_phase_one {
() => {
// Module: crate::header::map
// Provides: {"insert_phase_one"}
// Dependencies: {}
macro_rules ! insert_phase_one { ($ map : ident , $ key : expr , $ probe : ident , $ pos : ident , $ hash : ident , $ danger : ident , $ vacant : expr , $ occupied : expr , $ robinhood : expr) => { { let $ hash = hash_elem_using (&$ map . danger , &$ key) ; let mut $ probe = desired_pos ($ map . mask , $ hash) ; let mut dist = 0 ; let ret ; probe_loop ! ('probe : $ probe < $ map . indices . len () , { if let Some (($ pos , entry_hash)) = $ map . indices [$ probe] . resolve () { let their_dist = probe_distance ($ map . mask , entry_hash , $ probe) ; if their_dist < dist { let $ danger = dist >= FORWARD_SHIFT_THRESHOLD && !$ map . danger . is_red () ; ret = $ robinhood ; break 'probe ; } else if entry_hash == $ hash && $ map . entries [$ pos] . key == $ key { ret = $ occupied ; break 'probe ; } } else { let $ danger = dist >= FORWARD_SHIFT_THRESHOLD && !$ map . danger . is_red () ; ret = $ vacant ; break 'probe ; } dist += 1 ; }) ; ret } } }
};
}
