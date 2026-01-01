/* FP:worker_local.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_USE_0001
/* FP:worker_local.rs-0002 */ use std :: cell :: { Cell , OnceCell } ;
/* FP:worker_local.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_USE_0002
/* FP:worker_local.rs-0004 */ use std :: num :: NonZero ;
/* FP:worker_local.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_USE_0003
/* FP:worker_local.rs-0006 */ use std :: ops :: Deref ;
/* FP:worker_local.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_USE_0004
/* FP:worker_local.rs-0008 */ use std :: ptr ;
/* FP:worker_local.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_USE_0005
/* FP:worker_local.rs-0010 */ use std :: sync :: Arc ;
/* FP:worker_local.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_USE_0006
/* FP:worker_local.rs-0012 */ use parking_lot :: Mutex ;
/* FP:worker_local.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_USE_0007
/* FP:worker_local.rs-0014 */ use crate :: outline ;
/* FP:worker_local.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_USE_0008
/* FP:worker_local.rs-0016 */ use crate :: sync :: CacheAligned ;
/* FP:worker_local.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_STRUCT_0009
/* FP:worker_local.rs-0018 */ # [doc = " A pointer to the `RegistryData` which uniquely identifies a registry."] # [doc = " This identifier can be reused if the registry gets freed."] # [derive (Clone , Copy , PartialEq)] struct RegistryId (* const RegistryData) ;
/* FP:worker_local.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_IMPL_0010
/* FP:worker_local.rs-0020 */ impl RegistryId { # [inline (always)] # [doc = " Verifies that the current thread is associated with the registry and returns its unique"] # [doc = " index within the registry. This panics if the current thread is not associated with this"] # [doc = " registry."] # [doc = ""] # [doc = " Note that there's a race possible where the identifier in `THREAD_DATA` could be reused"] # [doc = " so this can succeed from a different registry."] fn verify (self) -> usize { let (id , index) = THREAD_DATA . with (| data | (data . registry_id . get () , data . index . get ())) ; if id == self { index } else { outline (| | panic ! ("Unable to verify registry association")) } } }
/* FP:worker_local.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_STRUCT_0011
/* FP:worker_local.rs-0022 */ struct RegistryData { thread_limit : NonZero < usize > , threads : Mutex < usize > , }
/* FP:worker_local.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_STRUCT_0012
/* FP:worker_local.rs-0024 */ # [doc = " Represents a list of threads which can access worker locals."] # [derive (Clone)] pub struct Registry (Arc < RegistryData >) ;
/* FP:worker_local.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_MACRO_0013
/* FP:worker_local.rs-0026 */ thread_local ! { # [doc = " The registry associated with the thread."] # [doc = " This allows the `WorkerLocal` type to clone the registry in its constructor."] static REGISTRY : OnceCell < Registry > = const { OnceCell :: new () } ; }
/* FP:worker_local.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_STRUCT_0014
/* FP:worker_local.rs-0028 */ struct ThreadData { registry_id : Cell < RegistryId > , index : Cell < usize > , }
/* FP:worker_local.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_MACRO_0015
/* FP:worker_local.rs-0030 */ thread_local ! { # [doc = " A thread local which contains the identifier of `REGISTRY` but allows for faster access."] # [doc = " It also holds the index of the current thread."] static THREAD_DATA : ThreadData = const { ThreadData { registry_id : Cell :: new (RegistryId (ptr :: null ())) , index : Cell :: new (0) , } } ; }
/* FP:worker_local.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_IMPL_0016
/* FP:worker_local.rs-0032 */ impl Registry { # [doc = " Creates a registry which can hold up to `thread_limit` threads."] pub fn new (thread_limit : NonZero < usize >) -> Self { Registry (Arc :: new (RegistryData { thread_limit , threads : Mutex :: new (0) })) } # [doc = " Gets the registry associated with the current thread. Panics if there's no such registry."] pub fn current () -> Self { REGISTRY . with (| registry | registry . get () . cloned () . expect ("No associated registry")) } # [doc = " Registers the current thread with the registry so worker locals can be used on it."] # [doc = " Panics if the thread limit is hit or if the thread already has an associated registry."] pub fn register (& self) { let mut threads = self . 0 . threads . lock () ; if * threads < self . 0 . thread_limit . get () { REGISTRY . with (| registry | { if registry . get () . is_some () { drop (threads) ; panic ! ("Thread already has a registry") ; } registry . set (self . clone ()) . ok () ; THREAD_DATA . with (| data | { data . registry_id . set (self . id ()) ; data . index . set (* threads) ; }) ; * threads += 1 ; }) ; } else { drop (threads) ; panic ! ("Thread limit reached") ; } } # [doc = " Gets the identifier of this registry."] fn id (& self) -> RegistryId { RegistryId (& * self . 0) } }
/* FP:worker_local.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_STRUCT_0017
/* FP:worker_local.rs-0034 */ # [doc = " Holds worker local values for each possible thread in a registry. You can only access the"] # [doc = " worker local value through the `Deref` impl on the registry associated with the thread it was"] # [doc = " created on. It will panic otherwise."] pub struct WorkerLocal < T > { locals : Box < [CacheAligned < T >] > , registry : Registry , }
/* FP:worker_local.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_IMPL_0018
/* FP:worker_local.rs-0036 */ unsafe impl < T : Send > Sync for WorkerLocal < T > { }
/* FP:worker_local.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_IMPL_0019
/* FP:worker_local.rs-0038 */ impl < T > WorkerLocal < T > { # [doc = " Creates a new worker local where the `initial` closure computes the"] # [doc = " value this worker local should take for each thread in the registry."] # [inline] pub fn new < F : FnMut (usize) -> T > (mut initial : F) -> WorkerLocal < T > { let registry = Registry :: current () ; WorkerLocal { locals : (0 .. registry . 0 . thread_limit . get ()) . map (| i | CacheAligned (initial (i))) . collect () , registry , } } # [doc = " Returns the worker-local values for each thread"] # [inline] pub fn into_inner (self) -> impl Iterator < Item = T > { self . locals . into_vec () . into_iter () . map (| local | local . 0) } }
/* FP:worker_local.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_IMPL_0020
/* FP:worker_local.rs-0040 */ impl < T > Deref for WorkerLocal < T > { type Target = T ; # [inline (always)] fn deref (& self) -> & T { unsafe { & self . locals . get_unchecked (self . registry . id () . verify ()) . 0 } } }
/* FP:worker_local.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_worker_local_IMPL_0021
/* FP:worker_local.rs-0042 */ impl < T : Default > Default for WorkerLocal < T > { fn default () -> Self { WorkerLocal :: new (| _ | T :: default ()) } }