mkmod!{empty, { 
                getname!(empty);
                getsrc!(empty);
                getpath!(empty);
                get_deps!(empty);
                get_crates!(empty);
                mkinclude!(empty);
                 
            }}
mkmod!{from_coroutine, { 
                getname!(from_coroutine);
                getsrc!(from_coroutine);
                getpath!(from_coroutine);
                get_deps!(from_coroutine);
                get_crates!(from_coroutine);
                mkinclude!(from_coroutine);
                 
            }}
mkmod!{from_fn, { 
                getname!(from_fn);
                getsrc!(from_fn);
                getpath!(from_fn);
                get_deps!(from_fn);
                get_crates!(from_fn);
                mkinclude!(from_fn);
                 
            }}
mkmod!{generator, { 
                getname!(generator);
                getsrc!(generator);
                getpath!(generator);
                get_deps!(generator);
                get_crates!(generator);
                mkinclude!(generator);
                 
            }}
mkmod!{once, { 
                getname!(once);
                getsrc!(once);
                getpath!(once);
                get_deps!(once);
                get_crates!(once);
                mkinclude!(once);
                 
            }}
mkmod!{once_with, { 
                getname!(once_with);
                getsrc!(once_with);
                getpath!(once_with);
                get_deps!(once_with);
                get_crates!(once_with);
                mkinclude!(once_with);
                 
            }}
mkmod!{repeat, { 
                getname!(repeat);
                getsrc!(repeat);
                getpath!(repeat);
                get_deps!(repeat);
                get_crates!(repeat);
                mkinclude!(repeat);
                 
            }}
mkmod!{repeat_n, { 
                getname!(repeat_n);
                getsrc!(repeat_n);
                getpath!(repeat_n);
                get_deps!(repeat_n);
                get_crates!(repeat_n);
                mkinclude!(repeat_n);
                 
            }}
mkmod!{repeat_with, { 
                getname!(repeat_with);
                getsrc!(repeat_with);
                getpath!(repeat_with);
                get_deps!(repeat_with);
                get_crates!(repeat_with);
                mkinclude!(repeat_with);
                 
            }}
mkmod!{successors, { 
                getname!(successors);
                getsrc!(successors);
                getpath!(successors);
                get_deps!(successors);
                get_crates!(successors);
                mkinclude!(successors);
                 
            }}
mkuse!{# [stable (feature = "iter_empty" , since = "1.2.0")] pub use self :: empty :: { Empty , empty } ;}
mkuse!{# [unstable (feature = "iter_from_coroutine" , issue = "43122" , reason = "coroutines are unstable")] pub use self :: from_coroutine :: { FromCoroutine , from_coroutine } ;}
mkuse!{# [stable (feature = "iter_from_fn" , since = "1.34.0")] pub use self :: from_fn :: { FromFn , from_fn } ;}
mkuse!{# [unstable (feature = "iter_macro" , issue = "142269" , reason = "generators are unstable")] pub use self :: generator :: iter ;}
mkuse!{# [stable (feature = "iter_once" , since = "1.2.0")] pub use self :: once :: { Once , once } ;}
mkuse!{# [stable (feature = "iter_once_with" , since = "1.43.0")] pub use self :: once_with :: { OnceWith , once_with } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: repeat :: { Repeat , repeat } ;}
mkuse!{# [stable (feature = "iter_repeat_n" , since = "1.82.0")] pub use self :: repeat_n :: { RepeatN , repeat_n } ;}
mkuse!{# [stable (feature = "iterator_repeat_with" , since = "1.28.0")] pub use self :: repeat_with :: { RepeatWith , repeat_with } ;}
mkuse!{# [stable (feature = "iter_successors" , since = "1.34.0")] pub use self :: successors :: { Successors , successors } ;}