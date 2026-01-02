mkmod!{addr, { 
                getname!(addr);
                getsrc!(addr);
                getpath!(addr);
                get_deps!(addr);
                get_crates!(addr);
                mkinclude!(addr);
                 
            }}
mkmod!{ancillary, { 
                getname!(ancillary);
                getsrc!(ancillary);
                getpath!(ancillary);
                get_deps!(ancillary);
                get_crates!(ancillary);
                mkinclude!(ancillary);
                 
            }}
mkmod!{datagram, { 
                getname!(datagram);
                getsrc!(datagram);
                getpath!(datagram);
                get_deps!(datagram);
                get_crates!(datagram);
                mkinclude!(datagram);
                 
            }}
mkmod!{listener, { 
                getname!(listener);
                getsrc!(listener);
                getpath!(listener);
                get_deps!(listener);
                get_crates!(listener);
                mkinclude!(listener);
                 
            }}
mkmod!{stream, { 
                getname!(stream);
                getsrc!(stream);
                getpath!(stream);
                get_deps!(stream);
                get_crates!(stream);
                mkinclude!(stream);
                 
            }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkmod!{ucred, { 
                getname!(ucred);
                getsrc!(ucred);
                getpath!(ucred);
                get_deps!(ucred);
                get_crates!(ucred);
                mkinclude!(ucred);
                 
            }}
mkuse!{# [stable (feature = "unix_socket" , since = "1.10.0")] pub use self :: addr :: * ;}
mkuse!{# [cfg (any (doc , target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub use self :: ancillary :: * ;}
mkuse!{# [stable (feature = "unix_socket" , since = "1.10.0")] pub use self :: datagram :: * ;}
mkuse!{# [stable (feature = "unix_socket" , since = "1.10.0")] pub use self :: listener :: * ;}
mkuse!{# [stable (feature = "unix_socket" , since = "1.10.0")] pub use self :: stream :: * ;}
mkuse!{# [cfg (any (target_os = "android" , target_os = "linux" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "netbsd" , target_os = "openbsd" , target_os = "nto" , target_vendor = "apple" , target_os = "cygwin" ,))] # [unstable (feature = "peer_credentials_unix_socket" , issue = "42839" , reason = "unstable")] pub use self :: ucred :: * ;}