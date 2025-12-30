// Generated macro for Executor (trait)
macro_rules! Depcrate_executorExecutor {
() => {
// Module: crate::executor
// Provides: {"Executor"}
// Dependencies: {}
# [doc = " Encapsulation of a value which has the ability to execute arbitrary code."] # [doc = ""] # [doc = " This trait is object safe and intended to be used through pointers like"] # [doc = " `Box` and `Arc."] pub trait Executor : Send + Sync + 'static { # [doc = " Executes the given closure `f`, perhaps on a different thread or"] # [doc = " deferred to a later time."] # [doc = ""] # [doc = " This method may not execute `f` immediately, but it will arrange for the"] # [doc = " callback to be invoked \"in the near future\"."] fn execute < F > (& self , f : F) where F : FnOnce () + Send + 'static , Self : Sized { self . execute_boxed (Box :: new (f)) } # [doc = " Object-safe method of the above interface used when implementing trait"] # [doc = " objects."] # [doc = ""] # [doc = " This should not be called direclty and instead `execute` should be used."] fn execute_boxed (& self , f : Box < ExecuteCallback >) ; }
};
}
