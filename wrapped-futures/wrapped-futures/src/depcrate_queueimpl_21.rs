// Generated macro for impl_21 (impl)
macro_rules! Depcrate_queueimpl_21 {
() => {
// Module: crate::queue
// Provides: {"impl_21"}
// Dependencies: {}
impl Queue { fn new () -> Self { let state = Rc :: new (QueueState { is_scheduled : Cell :: new (false) , tasks : RefCell :: new (VecDeque :: new ()) , }) ; let has_queue_microtask = js_sys :: global () . unchecked_into :: < Global > () . hasQueueMicrotask () . is_function () ; Self { promise : Promise :: resolve (& JsValue :: undefined ()) , closure : { let state = Rc :: clone (& state) ; Closure :: new (move | _ | state . run_all ()) } , state , has_queue_microtask , } } pub (crate) fn with < R > (f : impl FnOnce (& Self) -> R) -> R { use once_cell :: unsync :: Lazy ; struct Wrapper < T > (Lazy < T >) ; # [cfg (not (target_feature = "atomics"))] unsafe impl < T > Sync for Wrapper < T > { } # [cfg (not (target_feature = "atomics"))] unsafe impl < T > Send for Wrapper < T > { } # [cfg_attr (target_feature = "atomics" , thread_local)] static QUEUE : Wrapper < Queue > = Wrapper (Lazy :: new (Queue :: new)) ; f (& QUEUE . 0) } }
};
}
