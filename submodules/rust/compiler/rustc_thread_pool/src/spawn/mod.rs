mkuse!{use std :: mem ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use crate :: job :: * ;}
mkuse!{use crate :: registry :: Registry ;}
mkuse!{use crate :: tlv :: Tlv ;}
mkuse!{use crate :: unwind ;}

macro_rules! spawn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spawn in module {}", module_path!());
    };
}

mkfn!{
    spawn_introspect!();
    # [doc = " Puts the task into the Rayon threadpool's job queue in the \"static\""] # [doc = " or \"global\" scope. Just like a standard thread, this task is not"] # [doc = " tied to the current stack frame, and hence it cannot hold any"] # [doc = " references other than those with `'static` lifetime. If you want"] # [doc = " to spawn a task that references stack data, use [the `scope()`"] # [doc = " function][scope] to create a scope."] # [doc = ""] # [doc = " [scope]: fn.scope.html"] # [doc = ""] # [doc = " Since tasks spawned with this function cannot hold references into"] # [doc = " the enclosing stack frame, you almost certainly want to use a"] # [doc = " `move` closure as their argument (otherwise, the closure will"] # [doc = " typically hold references to any variables from the enclosing"] # [doc = " function that you happen to use)."] # [doc = ""] # [doc = " This API assumes that the closure is executed purely for its"] # [doc = " side-effects (i.e., it might send messages, modify data protected"] # [doc = " by a mutex, or some such thing)."] # [doc = ""] # [doc = " There is no guaranteed order of execution for spawns, given that"] # [doc = " other threads may steal tasks at any time. However, they are"] # [doc = " generally prioritized in a LIFO order on the thread from which"] # [doc = " they were spawned. Other threads always steal from the other end of"] # [doc = " the deque, like FIFO order. The idea is that \"recent\" tasks are"] # [doc = " most likely to be fresh in the local CPU's cache, while other"] # [doc = " threads can steal older \"stale\" tasks. For an alternate approach,"] # [doc = " consider [`spawn_fifo()`] instead."] # [doc = ""] # [doc = " [`spawn_fifo()`]: fn.spawn_fifo.html"] # [doc = ""] # [doc = " # Panic handling"] # [doc = ""] # [doc = " If this closure should panic, the resulting panic will be"] # [doc = " propagated to the panic handler registered in the `ThreadPoolBuilder`,"] # [doc = " if any. See [`ThreadPoolBuilder::panic_handler()`][ph] for more"] # [doc = " details."] # [doc = ""] # [doc = " [ph]: struct.ThreadPoolBuilder.html#method.panic_handler"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " This code creates a Rayon task that increments a global counter."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use rustc_thread_pool as rayon;"] # [doc = " use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};"] # [doc = ""] # [doc = " static GLOBAL_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;"] # [doc = ""] # [doc = " rayon::spawn(move || {"] # [doc = "     GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);"] # [doc = " });"] # [doc = " ```"] pub fn spawn < F > (func : F) where F : FnOnce () + Send + 'static , { unsafe { spawn_in (func , & Registry :: current ()) } }
}

macro_rules! spawn_in_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spawn_in in module {}", module_path!());
    };
}

mkfn!{
    spawn_in_introspect!();
    # [doc = " Spawns an asynchronous job in `registry.`"] # [doc = ""] # [doc = " Unsafe because `registry` must not yet have terminated."] pub (super) unsafe fn spawn_in < F > (func : F , registry : & Arc < Registry >) where F : FnOnce () + Send + 'static , { let abort_guard = unwind :: AbortIfPanic ; let job_ref = unsafe { spawn_job (func , registry) } ; registry . inject_or_push (job_ref) ; mem :: forget (abort_guard) ; }
}

macro_rules! spawn_job_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spawn_job in module {}", module_path!());
    };
}

mkfn!{
    spawn_job_introspect!();
    unsafe fn spawn_job < F > (func : F , registry : & Arc < Registry >) -> JobRef where F : FnOnce () + Send + 'static , { registry . increment_terminate_count () ; HeapJob :: new (Tlv :: null () , { let registry = Arc :: clone (registry) ; move | _ | { registry . catch_unwind (func) ; registry . terminate () ; } }) . into_static_job_ref () }
}

macro_rules! spawn_fifo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spawn_fifo in module {}", module_path!());
    };
}

mkfn!{
    spawn_fifo_introspect!();
    # [doc = " Fires off a task into the Rayon threadpool in the \"static\" or"] # [doc = " \"global\" scope. Just like a standard thread, this task is not"] # [doc = " tied to the current stack frame, and hence it cannot hold any"] # [doc = " references other than those with `'static` lifetime. If you want"] # [doc = " to spawn a task that references stack data, use [the `scope_fifo()`"] # [doc = " function](fn.scope_fifo.html) to create a scope."] # [doc = ""] # [doc = " The behavior is essentially the same as [the `spawn`"] # [doc = " function](fn.spawn.html), except that calls from the same thread"] # [doc = " will be prioritized in FIFO order. This is similar to the now-"] # [doc = " deprecated [`breadth_first`] option, except the effect is isolated"] # [doc = " to relative `spawn_fifo` calls, not all threadpool tasks."] # [doc = ""] # [doc = " For more details on this design, see Rayon [RFC #1]."] # [doc = ""] # [doc = " [`breadth_first`]: struct.ThreadPoolBuilder.html#method.breadth_first"] # [doc = " [RFC #1]: https://github.com/rayon-rs/rfcs/blob/master/accepted/rfc0001-scope-scheduling.md"] # [doc = ""] # [doc = " # Panic handling"] # [doc = ""] # [doc = " If this closure should panic, the resulting panic will be"] # [doc = " propagated to the panic handler registered in the `ThreadPoolBuilder`,"] # [doc = " if any. See [`ThreadPoolBuilder::panic_handler()`][ph] for more"] # [doc = " details."] # [doc = ""] # [doc = " [ph]: struct.ThreadPoolBuilder.html#method.panic_handler"] pub fn spawn_fifo < F > (func : F) where F : FnOnce () + Send + 'static , { unsafe { spawn_fifo_in (func , & Registry :: current ()) } }
}

macro_rules! spawn_fifo_in_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spawn_fifo_in in module {}", module_path!());
    };
}

mkfn!{
    spawn_fifo_in_introspect!();
    # [doc = " Spawns an asynchronous FIFO job in `registry.`"] # [doc = ""] # [doc = " Unsafe because `registry` must not yet have terminated."] pub (super) unsafe fn spawn_fifo_in < F > (func : F , registry : & Arc < Registry >) where F : FnOnce () + Send + 'static , { let abort_guard = unwind :: AbortIfPanic ; let job_ref = unsafe { spawn_job (func , registry) } ; match registry . current_thread () { Some (worker) => unsafe { worker . push_fifo (job_ref) } , None => registry . inject (job_ref) , } mem :: forget (abort_guard) ; }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}