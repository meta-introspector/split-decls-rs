mkmod!{poll, { 
                getname!(poll);
                getsrc!(poll);
                getpath!(poll);
                get_deps!(poll);
                get_crates!(poll);
                mkinclude!(poll);
                 
            }}
mkuse!{# [stable (feature = "futures_api" , since = "1.36.0")] pub use self :: poll :: Poll ;}
mkmod!{wake, { 
                getname!(wake);
                getsrc!(wake);
                getpath!(wake);
                get_deps!(wake);
                get_crates!(wake);
                mkinclude!(wake);
                 
            }}
mkuse!{# [stable (feature = "futures_api" , since = "1.36.0")] pub use self :: wake :: { Context , ContextBuilder , LocalWaker , RawWaker , RawWakerVTable , Waker } ;}
mkmod!{ready, { 
                getname!(ready);
                getsrc!(ready);
                getpath!(ready);
                get_deps!(ready);
                get_crates!(ready);
                mkinclude!(ready);
                 
            }}
mkuse!{# [stable (feature = "ready_macro" , since = "1.64.0")] pub use ready :: ready ;}