mkmod!{connection, { 
                getname!(connection);
                getsrc!(connection);
                getpath!(connection);
                get_deps!(connection);
                get_crates!(connection);
                mkinclude!(connection);
                 
            }}
mkuse!{pub use connection :: * ;}