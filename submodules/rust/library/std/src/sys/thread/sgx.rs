mkuse!{use crate :: io ;}
mkuse!{use crate :: sys :: pal :: abi :: { thread , usercalls } ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{mkstruct!{pub struct Thread (task_queue :: JoinHandle) ;}}
mkitem!{pub const DEFAULT_MIN_STACK_SIZE : usize = 4096 ;}
mkuse!{pub use self :: task_queue :: JoinNotifier ;}
mkmod!{task_queue, { 
                getname!(task_queue);
                getsrc!(task_queue);
                getpath!(task_queue);
                get_deps!(task_queue);
                get_crates!(task_queue);
                mkinclude!(task_queue);
                mkuse!{use super :: wait_notify ;}
mkuse!{use crate :: sync :: { Mutex , MutexGuard } ;}
mkitem!{pub type JoinHandle = wait_notify :: Waiter ;}
mkitem!{mkstruct!{pub struct JoinNotifier (Option < wait_notify :: Notifier >) ;}}
mkitem!{mkimpl!{impl Drop for JoinNotifier { fn drop (& mut self) { self . 0 . take () . unwrap () . notify () ; } }}}
mkitem!{mkstruct!{pub (super) struct Task { p : Box < dyn FnOnce () + Send > , done : JoinNotifier , }}}
mkitem!{mkimpl!{impl Task { pub (super) fn new (p : Box < dyn FnOnce () + Send >) -> (Task , JoinHandle) { let (done , recv) = wait_notify :: new () ; let done = JoinNotifier (Some (done)) ; (Task { p , done } , recv) } pub (super) fn run (self) -> JoinNotifier { (self . p) () ; self . done } }}}
mkitem!{# [cfg_attr (test , linkage = "available_externally")] # [unsafe (export_name = "_ZN16__rust_internals3std3sys3pal3sgx6thread10TASK_QUEUEE")] static TASK_QUEUE : Mutex < Vec < Task > > = Mutex :: new (Vec :: new ()) ;}

macro_rules! lock_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lock in module {}", module_path!());
    };
}

mkfn!{
    lock_introspect!();
    pub (super) fn lock () -> MutexGuard < 'static , Vec < Task > > { TASK_QUEUE . lock () . unwrap () }
} 
            }}
mkmod!{wait_notify, { 
                getname!(wait_notify);
                getsrc!(wait_notify);
                getpath!(wait_notify);
                get_deps!(wait_notify);
                get_crates!(wait_notify);
                mkinclude!(wait_notify);
                mkuse!{use crate :: pin :: Pin ;}
mkuse!{use crate :: sync :: Arc ;}
mkuse!{use crate :: sys :: sync :: Parker ;}
mkitem!{mkstruct!{pub struct Notifier (Arc < Parker >) ;}}
mkitem!{mkimpl!{impl Notifier { # [doc = " Notify the waiter. The waiter is either notified right away (if"] # [doc = " currently blocked in `Waiter::wait()`) or later when it calls the"] # [doc = " `Waiter::wait()` method."] pub fn notify (self) { Pin :: new (& * self . 0) . unpark () } }}}
mkitem!{mkstruct!{pub struct Waiter (Arc < Parker >) ;}}
mkitem!{mkimpl!{impl Waiter { # [doc = " Wait for a notification. If `Notifier::notify()` has already been"] # [doc = " called, this will return immediately, otherwise the current thread"] # [doc = " is blocked until notified."] pub fn wait (self) { unsafe { Pin :: new (& * self . 0) . park () } } }}}

macro_rules! new_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new in module {}", module_path!());
    };
}

mkfn!{
    new_introspect!();
    pub fn new () -> (Notifier , Waiter) { let inner = Arc :: new (Parker :: new ()) ; (Notifier (inner . clone ()) , Waiter (inner)) }
} 
            }}
mkitem!{mkimpl!{impl Thread { pub unsafe fn new (_stack : usize , _name : Option < & str > , p : Box < dyn FnOnce () + Send > ,) -> io :: Result < Thread > { let mut queue_lock = task_queue :: lock () ; unsafe { usercalls :: launch_thread () ? } ; let (task , handle) = task_queue :: Task :: new (p) ; queue_lock . push (task) ; Ok (Thread (handle)) } pub (crate) fn entry () -> JoinNotifier { let mut pending_tasks = task_queue :: lock () ; let task = rtunwrap ! (Some , pending_tasks . pop ()) ; drop (pending_tasks) ; task . run () } pub fn join (self) { self . 0 . wait () ; } }}}

macro_rules! current_os_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_os_id in module {}", module_path!());
    };
}

mkfn!{
    current_os_id_introspect!();
    pub fn current_os_id () -> Option < u64 > { Some (thread :: current () . addr () . get () as u64) }
}

macro_rules! sleep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep in module {}", module_path!());
    };
}

mkfn!{
    sleep_introspect!();
    pub fn sleep (dur : Duration) { usercalls :: wait_timeout (0 , dur , | | true) ; }
}

macro_rules! yield_now_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function yield_now in module {}", module_path!());
    };
}

mkfn!{
    yield_now_introspect!();
    pub fn yield_now () { let wait_error = rtunwrap ! (Err , usercalls :: wait (0 , usercalls :: raw :: WAIT_NO)) ; rtassert ! (wait_error . kind () == io :: ErrorKind :: WouldBlock) ; }
}