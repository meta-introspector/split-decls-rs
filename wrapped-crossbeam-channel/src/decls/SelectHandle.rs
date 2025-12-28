macro_rules! deps {
    () => {
        Operation!();
        Context!();
        Token!();
    };
}

macro_rules! SelectHandle {
    () => {
        deps!();
        # [doc = " A receiver or a sender that can participate in select."] # [doc = ""] # [doc = " This is a handle that assists select in executing an operation, registration, deciding on the"] # [doc = " appropriate deadline for blocking, etc."] pub trait SelectHandle { # [doc = " Attempts to select an operation and returns `true` on success."] fn try_select (& self , token : & mut Token) -> bool ; # [doc = " Returns a deadline for an operation, if there is one."] fn deadline (& self) -> Option < Instant > ; # [doc = " Registers an operation for execution and returns `true` if it is now ready."] fn register (& self , oper : Operation , cx : & Context) -> bool ; # [doc = " Unregisters an operation for execution."] fn unregister (& self , oper : Operation) ; # [doc = " Attempts to select an operation the thread got woken up for and returns `true` on success."] fn accept (& self , token : & mut Token , cx : & Context) -> bool ; # [doc = " Returns `true` if an operation can be executed without blocking."] fn is_ready (& self) -> bool ; # [doc = " Registers an operation for readiness notification and returns `true` if it is now ready."] fn watch (& self , oper : Operation , cx : & Context) -> bool ; # [doc = " Unregisters an operation for readiness notification."] fn unwatch (& self , oper : Operation) ; }
    };
}

SelectHandle!();