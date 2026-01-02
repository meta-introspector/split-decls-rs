mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: ip_addr :: { IpAddr , Ipv4Addr , Ipv6Addr , Ipv6MulticastScope } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: parser :: AddrParseError ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: socket_addr :: { SocketAddr , SocketAddrV4 , SocketAddrV6 } ;}
mkmod!{display_buffer, { 
                getname!(display_buffer);
                getsrc!(display_buffer);
                getpath!(display_buffer);
                get_deps!(display_buffer);
                get_crates!(display_buffer);
                mkinclude!(display_buffer);
                 
            }}
mkmod!{ip_addr, { 
                getname!(ip_addr);
                getsrc!(ip_addr);
                getpath!(ip_addr);
                get_deps!(ip_addr);
                get_crates!(ip_addr);
                mkinclude!(ip_addr);
                 
            }}
mkmod!{parser, { 
                getname!(parser);
                getsrc!(parser);
                getpath!(parser);
                get_deps!(parser);
                get_crates!(parser);
                mkinclude!(parser);
                 
            }}
mkmod!{socket_addr, { 
                getname!(socket_addr);
                getsrc!(socket_addr);
                getpath!(socket_addr);
                get_deps!(socket_addr);
                get_crates!(socket_addr);
                mkinclude!(socket_addr);
                 
            }}