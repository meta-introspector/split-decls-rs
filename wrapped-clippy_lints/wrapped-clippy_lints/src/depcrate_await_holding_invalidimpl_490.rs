// Generated macro for impl_490 (impl)
macro_rules! Depcrate_await_holding_invalidimpl_490 {
() => {
// Module: crate::await_holding_invalid
// Provides: {"impl_490"}
// Dependencies: {}
impl AwaitHolding { fn check_interior_types (& self , cx : & LateContext < '_ > , coroutine : & CoroutineLayout < '_ >) { for (ty_index , ty_cause) in coroutine . field_tys . iter_enumerated () { if let rustc_middle :: ty :: Adt (adt , _) = ty_cause . ty . kind () { let await_points = | | { coroutine . variant_source_info . iter_enumerated () . filter_map (| (variant , source_info) | { coroutine . variant_fields [variant] . raw . contains (& ty_index) . then_some (source_info . span) }) . collect :: < Vec < _ > > () } ; if is_mutex_guard (cx , adt . did ()) { span_lint_and_then (cx , AWAIT_HOLDING_LOCK , ty_cause . source_info . span , "this `MutexGuard` is held across an await point" , | diag | { diag . help ("consider using an async-aware `Mutex` type or ensuring the \
                                `MutexGuard` is dropped before calling `await`" ,) ; diag . span_note (await_points () , "these are all the await points this lock is held through" ,) ; } ,) ; } else if is_refcell_ref (cx , adt . did ()) { span_lint_and_then (cx , AWAIT_HOLDING_REFCELL_REF , ty_cause . source_info . span , "this `RefCell` reference is held across an await point" , | diag | { diag . help ("ensure the reference is dropped before calling `await`") ; diag . span_note (await_points () , "these are all the await points this reference is held through" ,) ; } ,) ; } else if let Some (& (path , disallowed_path)) = self . def_ids . get (& adt . did ()) { emit_invalid_type (cx , ty_cause . source_info . span , path , disallowed_path) ; } } } } }
};
}
