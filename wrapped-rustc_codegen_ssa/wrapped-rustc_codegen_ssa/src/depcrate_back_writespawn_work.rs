// Generated macro for spawn_work (function)
macro_rules! Depcrate_back_writespawn_work {
() => {
// Module: crate::back::write
// Provides: {"spawn_work"}
// Dependencies: {}
fn spawn_work < 'a , B : ExtraBackendMethods > (cgcx : & 'a CodegenContext < B > , coordinator_send : Sender < Message < B > > , llvm_start_time : & mut Option < VerboseTimingGuard < 'a > > , work : WorkItem < B > ,) { if llvm_start_time . is_none () { * llvm_start_time = Some (cgcx . prof . verbose_generic_activity ("LLVM_passes")) ; } let cgcx = cgcx . clone () ; B :: spawn_named_thread (cgcx . time_trace , work . short_description () , move | | { let result = std :: panic :: catch_unwind (AssertUnwindSafe (| | match work { WorkItem :: Optimize (m) => execute_optimize_work_item (& cgcx , m) , WorkItem :: CopyPostLtoArtifacts (m) => execute_copy_from_cache_work_item (& cgcx , m) , WorkItem :: FatLto { exported_symbols_for_lto , each_linked_rlib_for_lto , needs_fat_lto , import_only_modules , } => execute_fat_lto_work_item (& cgcx , & exported_symbols_for_lto , & each_linked_rlib_for_lto , needs_fat_lto , import_only_modules ,) , WorkItem :: ThinLto (m) => execute_thin_lto_work_item (& cgcx , m) , })) ; let msg = match result { Ok (result) => Message :: WorkItem :: < B > { result : Ok (result) } , Err (err) if err . is :: < FatalErrorMarker > () => { Message :: WorkItem :: < B > { result : Err (Some (WorkerFatalError)) } } Err (_) => Message :: WorkItem :: < B > { result : Err (None) } , } ; drop (coordinator_send . send (msg)) ; }) . expect ("failed to spawn work thread") ; }
};
}
