mkuse!{use crate :: fmt :: { self , Debug } ;}
mkuse!{use crate :: future :: Future ;}
mkuse!{use crate :: marker ;}
mkuse!{use crate :: pin :: Pin ;}
mkuse!{use crate :: task :: { Context , Poll } ;}
mkitem!{mkstruct!{# [doc = " Creates a future which never resolves, representing a computation that never"] # [doc = " finishes."] # [doc = ""] # [doc = " This `struct` is created by [`pending()`]. See its"] # [doc = " documentation for more."] # [stable (feature = "future_readiness_fns" , since = "1.48.0")] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Pending < T > { _data : marker :: PhantomData < fn () -> T > , }}}

macro_rules! pending_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pending in module {}", module_path!());
    };
}

mkfn!{
    pending_introspect!();
    # [doc = " Creates a future which never resolves, representing a computation that never"] # [doc = " finishes."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::future;"] # [doc = ""] # [doc = " # async fn run() {"] # [doc = " let future = future::pending();"] # [doc = " let () = future.await;"] # [doc = " unreachable!();"] # [doc = " # }"] # [doc = " ```"] # [stable (feature = "future_readiness_fns" , since = "1.48.0")] pub fn pending < T > () -> Pending < T > { Pending { _data : marker :: PhantomData } }
}
mkitem!{mkimpl!{# [stable (feature = "future_readiness_fns" , since = "1.48.0")] impl < T > Future for Pending < T > { type Output = T ; fn poll (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < T > { Poll :: Pending } }}}
mkitem!{mkimpl!{# [stable (feature = "future_readiness_fns" , since = "1.48.0")] impl < T > Debug for Pending < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Pending") . finish () } }}}
mkitem!{mkimpl!{# [stable (feature = "future_readiness_fns" , since = "1.48.0")] impl < T > Clone for Pending < T > { fn clone (& self) -> Self { pending () } }}}