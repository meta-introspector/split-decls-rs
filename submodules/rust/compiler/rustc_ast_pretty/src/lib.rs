mkmod!{helpers, { 
                getname!(helpers);
                getsrc!(helpers);
                getpath!(helpers);
                get_deps!(helpers);
                get_crates!(helpers);
                mkinclude!(helpers);
                 
            }}
mkmod!{pp, { 
                getname!(pp);
                getsrc!(pp);
                getpath!(pp);
                get_deps!(pp);
                get_crates!(pp);
                mkinclude!(pp);
                 
            }}
mkmod!{pprust, { 
                getname!(pprust);
                getsrc!(pprust);
                getpath!(pprust);
                get_deps!(pprust);
                get_crates!(pprust);
                mkinclude!(pprust);
                 
            }}