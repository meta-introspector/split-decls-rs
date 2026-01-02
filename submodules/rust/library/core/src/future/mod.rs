mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: task :: Context ;}
mkmod!{async_drop, { 
                getname!(async_drop);
                getsrc!(async_drop);
                getpath!(async_drop);
                get_deps!(async_drop);
                get_crates!(async_drop);
                mkinclude!(async_drop);
                 
            }}
mkmod!{future, { 
                getname!(future);
                getsrc!(future);
                getpath!(future);
                get_deps!(future);
                get_crates!(future);
                mkinclude!(future);
                 
            }}
mkmod!{into_future, { 
                getname!(into_future);
                getsrc!(into_future);
                getpath!(into_future);
                get_deps!(into_future);
                get_crates!(into_future);
                mkinclude!(into_future);
                 
            }}
mkmod!{join, { 
                getname!(join);
                getsrc!(join);
                getpath!(join);
                get_deps!(join);
                get_crates!(join);
                mkinclude!(join);
                 
            }}
mkmod!{pending, { 
                getname!(pending);
                getsrc!(pending);
                getpath!(pending);
                get_deps!(pending);
                get_crates!(pending);
                mkinclude!(pending);
                 
            }}
mkmod!{poll_fn, { 
                getname!(poll_fn);
                getsrc!(poll_fn);
                getpath!(poll_fn);
                get_deps!(poll_fn);
                get_crates!(poll_fn);
                mkinclude!(poll_fn);
                 
            }}
mkmod!{ready, { 
                getname!(ready);
                getsrc!(ready);
                getpath!(ready);
                get_deps!(ready);
                get_crates!(ready);
                mkinclude!(ready);
                 
            }}
mkuse!{# [unstable (feature = "async_drop" , issue = "126482")] pub use async_drop :: { AsyncDrop , async_drop_in_place } ;}
mkuse!{# [stable (feature = "into_future" , since = "1.64.0")] pub use into_future :: IntoFuture ;}
mkuse!{# [stable (feature = "future_readiness_fns" , since = "1.48.0")] pub use pending :: { Pending , pending } ;}
mkuse!{# [stable (feature = "future_poll_fn" , since = "1.64.0")] pub use poll_fn :: { PollFn , poll_fn } ;}
mkuse!{# [stable (feature = "future_readiness_fns" , since = "1.48.0")] pub use ready :: { Ready , ready } ;}
mkuse!{# [stable (feature = "futures_api" , since = "1.36.0")] pub use self :: future :: Future ;}
mkuse!{# [unstable (feature = "future_join" , issue = "91642")] pub use self :: join :: join ;}
mkitem!{mkstruct!{# [doc = " This type is needed because:"] # [doc = ""] # [doc = " a) Coroutines cannot implement `for<'a, 'b> Coroutine<&'a mut Context<'b>>`, so we need to pass"] # [doc = "    a raw pointer (see <https://github.com/rust-lang/rust/issues/68923>)."] # [doc = " b) Raw pointers and `NonNull` aren't `Send` or `Sync`, so that would make every single future"] # [doc = "    non-Send/Sync as well, and we don't want that."] # [doc = ""] # [doc = " It also simplifies the HIR lowering of `.await`."] # [lang = "ResumeTy"] # [doc (hidden)] # [unstable (feature = "gen_future" , issue = "none")] # [derive (Debug , Copy , Clone)] pub struct ResumeTy (NonNull < Context < 'static > >) ;}}
mkitem!{mkimpl!{# [unstable (feature = "gen_future" , issue = "none")] unsafe impl Send for ResumeTy { }}}
mkitem!{mkimpl!{# [unstable (feature = "gen_future" , issue = "none")] unsafe impl Sync for ResumeTy { }}}

macro_rules! get_context_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_context in module {}", module_path!());
    };
}

mkfn!{
    get_context_introspect!();
    # [lang = "get_context"] # [doc (hidden)] # [unstable (feature = "gen_future" , issue = "none")] # [must_use] # [inline] pub unsafe fn get_context < 'a , 'b > (cx : ResumeTy) -> & 'a mut Context < 'b > { unsafe { & mut * cx . 0 . as_ptr () . cast () } }
}