mkmod!{dns, { 
                getname!(dns);
                getsrc!(dns);
                getpath!(dns);
                get_deps!(dns);
                get_crates!(dns);
                mkinclude!(dns);
                 
            }}
mkmod!{tcpstream, { 
                getname!(tcpstream);
                getsrc!(tcpstream);
                getpath!(tcpstream);
                get_deps!(tcpstream);
                get_crates!(tcpstream);
                mkinclude!(tcpstream);
                 
            }}
mkuse!{pub use tcpstream :: * ;}
mkmod!{tcplistener, { 
                getname!(tcplistener);
                getsrc!(tcplistener);
                getpath!(tcplistener);
                get_deps!(tcplistener);
                get_crates!(tcplistener);
                mkinclude!(tcplistener);
                 
            }}
mkuse!{pub use tcplistener :: * ;}
mkmod!{udp, { 
                getname!(udp);
                getsrc!(udp);
                getpath!(udp);
                get_deps!(udp);
                get_crates!(udp);
                mkinclude!(udp);
                 
            }}
mkuse!{pub use udp :: * ;}
mkitem!{mkenum!{# [repr (C)] # [derive (Debug)] enum NetError { Unaddressable = 1 , SocketInUse = 2 , Invalid = 4 , LibraryError = 6 , TimedOut = 8 , WouldBlock = 9 , }}}
mkitem!{mkstruct!{# [repr (C , align (4096))] struct ConnectRequest { raw : [u8 ; 4096] , }}}
mkitem!{mkstruct!{# [repr (C , align (4096))] struct SendData { raw : [u8 ; 4096] , }}}
mkitem!{mkstruct!{# [repr (C , align (4096))] pub struct ReceiveData { raw : [u8 ; 4096] , }}}
mkitem!{mkstruct!{# [repr (C , align (4096))] pub struct GetAddress { raw : [u8 ; 4096] , }}}
mkuse!{pub use dns :: LookupHost ;}