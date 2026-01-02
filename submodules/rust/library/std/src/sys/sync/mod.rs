mkmod!{condvar, { 
                getname!(condvar);
                getsrc!(condvar);
                getpath!(condvar);
                get_deps!(condvar);
                get_crates!(condvar);
                mkinclude!(condvar);
                 
            }}
mkmod!{mutex, { 
                getname!(mutex);
                getsrc!(mutex);
                getpath!(mutex);
                get_deps!(mutex);
                get_crates!(mutex);
                mkinclude!(mutex);
                 
            }}
mkmod!{once, { 
                getname!(once);
                getsrc!(once);
                getpath!(once);
                get_deps!(once);
                get_crates!(once);
                mkinclude!(once);
                 
            }}
mkmod!{once_box, { 
                getname!(once_box);
                getsrc!(once_box);
                getpath!(once_box);
                get_deps!(once_box);
                get_crates!(once_box);
                mkinclude!(once_box);
                 
            }}
mkmod!{rwlock, { 
                getname!(rwlock);
                getsrc!(rwlock);
                getpath!(rwlock);
                get_deps!(rwlock);
                get_crates!(rwlock);
                mkinclude!(rwlock);
                 
            }}
mkmod!{thread_parking, { 
                getname!(thread_parking);
                getsrc!(thread_parking);
                getpath!(thread_parking);
                get_deps!(thread_parking);
                get_crates!(thread_parking);
                mkinclude!(thread_parking);
                 
            }}
mkuse!{pub use condvar :: Condvar ;}
mkuse!{pub use mutex :: Mutex ;}
mkuse!{pub use once :: { Once , OnceState } ;}
mkuse!{# [allow (unused)] use once_box :: OnceBox ;}
mkuse!{pub use rwlock :: RwLock ;}
mkuse!{pub use thread_parking :: Parker ;}