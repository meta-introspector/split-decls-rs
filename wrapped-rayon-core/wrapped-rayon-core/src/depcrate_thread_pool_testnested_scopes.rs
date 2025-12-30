// Generated macro for nested_scopes (function)
macro_rules! Depcrate_thread_pool_testnested_scopes {
() => {
// Module: crate::thread_pool::test
// Provides: {"nested_scopes"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn nested_scopes () { fn nest < 'scope , OP > (pools : & [ThreadPool] , scopes : Vec < & Scope < 'scope > > , op : OP) where OP : FnOnce (& [& Scope < 'scope >]) + Send , { if let Some ((pool , tail)) = pools . split_first () { pool . scope (move | s | { let mut scopes = scopes ; scopes . push (s) ; nest (tail , scopes , op) }) } else { (op) (& scopes) } } let pools : Vec < _ > = (0 .. 10) . map (| _ | ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap ()) . collect () ; let counter = AtomicUsize :: new (0) ; nest (& pools , vec ! [] , | scopes | { for & s in scopes { s . spawn (| _ | { counter . fetch_add (1 , Ordering :: Relaxed) ; }) ; } }) ; assert_eq ! (counter . into_inner () , pools . len ()) ; }
};
}
