mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{# [stable (feature = "ip_addr" , since = "1.7.0")] pub use core :: net :: IpAddr ;}
mkuse!{# [unstable (feature = "ip" , issue = "27709")] pub use core :: net :: Ipv6MulticastScope ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: net :: { Ipv4Addr , Ipv6Addr } ;}