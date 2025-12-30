// Generated macro for steal (function)
macro_rules! Depcratesteal {
() => {
// Module: crate
// Provides: {"steal"}
// Dependencies: {}
# [doc = " Steals some items from one queue into another."] fn steal < T > (src : & ConcurrentQueue < T > , dest : & ConcurrentQueue < T >) { let mut count = (src . len () + 1) / 2 ; if count > 0 { if let Some (cap) = dest . capacity () { count = count . min (cap - dest . len ()) ; } for _ in 0 .. count { if let Ok (t) = src . pop () { assert ! (dest . push (t) . is_ok ()) ; } else { break ; } } } }
};
}
