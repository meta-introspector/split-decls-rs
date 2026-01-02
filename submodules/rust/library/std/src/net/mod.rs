mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use core :: net :: AddrParseError ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: ip_addr :: { IpAddr , Ipv4Addr , Ipv6Addr , Ipv6MulticastScope } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: socket_addr :: { SocketAddr , SocketAddrV4 , SocketAddrV6 , ToSocketAddrs } ;}
mkuse!{# [unstable (feature = "tcplistener_into_incoming" , issue = "88373")] pub use self :: tcp :: IntoIncoming ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: tcp :: { Incoming , TcpListener , TcpStream } ;}
mkuse!{# [stable (feature = "rust1" , since = "1.0.0")] pub use self :: udp :: UdpSocket ;}
mkmod!{ip_addr, { 
                getname!(ip_addr);
                getsrc!(ip_addr);
                getpath!(ip_addr);
                get_deps!(ip_addr);
                get_crates!(ip_addr);
                mkinclude!(ip_addr);
                 
            }}
mkmod!{socket_addr, { 
                getname!(socket_addr);
                getsrc!(socket_addr);
                getpath!(socket_addr);
                get_deps!(socket_addr);
                get_crates!(socket_addr);
                mkinclude!(socket_addr);
                 
            }}
mkmod!{tcp, { 
                getname!(tcp);
                getsrc!(tcp);
                getpath!(tcp);
                get_deps!(tcp);
                get_crates!(tcp);
                mkinclude!(tcp);
                 
            }}
mkmod!{test, { 
                getname!(test);
                getsrc!(test);
                getpath!(test);
                get_deps!(test);
                get_crates!(test);
                mkinclude!(test);
                 
            }}
mkmod!{udp, { 
                getname!(udp);
                getsrc!(udp);
                getpath!(udp);
                get_deps!(udp);
                get_crates!(udp);
                mkinclude!(udp);
                 
            }}
mkitem!{mkenum!{# [doc = " Possible values which can be passed to the [`TcpStream::shutdown`] method."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] # [stable (feature = "rust1" , since = "1.0.0")] pub enum Shutdown { # [doc = " The reading portion of the [`TcpStream`] should be shut down."] # [doc = ""] # [doc = " All currently blocked and future [reads] will return <code>[Ok]\\(0)</code>."] # [doc = ""] # [doc = " [reads]: crate::io::Read \"io::Read\""] # [stable (feature = "rust1" , since = "1.0.0")] Read , # [doc = " The writing portion of the [`TcpStream`] should be shut down."] # [doc = ""] # [doc = " All currently blocked and future [writes] will return an error."] # [doc = ""] # [doc = " [writes]: crate::io::Write \"io::Write\""] # [stable (feature = "rust1" , since = "1.0.0")] Write , # [doc = " Both the reading and the writing portions of the [`TcpStream`] should be shut down."] # [doc = ""] # [doc = " See [`Shutdown::Read`] and [`Shutdown::Write`] for more information."] # [stable (feature = "rust1" , since = "1.0.0")] Both , }}}