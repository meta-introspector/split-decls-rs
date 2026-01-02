mkuse!{use std :: any :: Any ;}
mkuse!{use std :: panic :: { self , AssertUnwindSafe } ;}
mkuse!{use std :: thread ;}

macro_rules! halt_unwinding_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function halt_unwinding in module {}", module_path!());
    };
}

mkfn!{
    halt_unwinding_introspect!();
    # [doc = " Executes `f` and captures any panic, translating that panic into a"] # [doc = " `Err` result. The assumption is that any panic will be propagated"] # [doc = " later with `resume_unwinding`, and hence `f` can be treated as"] # [doc = " exception safe."] pub (super) fn halt_unwinding < F , R > (func : F) -> thread :: Result < R > where F : FnOnce () -> R , { panic :: catch_unwind (AssertUnwindSafe (func)) }
}

macro_rules! resume_unwinding_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resume_unwinding in module {}", module_path!());
    };
}

mkfn!{
    resume_unwinding_introspect!();
    pub (super) fn resume_unwinding (payload : Box < dyn Any + Send >) -> ! { panic :: resume_unwind (payload) }
}
mkitem!{mkstruct!{pub (super) struct AbortIfPanic ;}}
mkitem!{mkimpl!{impl Drop for AbortIfPanic { fn drop (& mut self) { eprintln ! ("Rayon: detected unexpected panic; aborting") ; :: std :: process :: abort () ; } }}}