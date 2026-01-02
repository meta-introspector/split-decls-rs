mkmod!{async_iter, { 
                getname!(async_iter);
                getsrc!(async_iter);
                getpath!(async_iter);
                get_deps!(async_iter);
                get_crates!(async_iter);
                mkinclude!(async_iter);
                 
            }}
mkmod!{from_iter, { 
                getname!(from_iter);
                getsrc!(from_iter);
                getpath!(from_iter);
                get_deps!(from_iter);
                get_crates!(from_iter);
                mkinclude!(from_iter);
                 
            }}
mkuse!{pub use async_iter :: { AsyncIterator , IntoAsyncIterator } ;}
mkuse!{pub use from_iter :: { FromIter , from_iter } ;}