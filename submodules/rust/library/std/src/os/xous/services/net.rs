mkuse!{use core :: sync :: atomic :: { Atomic , AtomicU32 , Ordering } ;}
mkuse!{use crate :: os :: xous :: ffi :: Connection ;}
mkuse!{use crate :: os :: xous :: services :: connect ;}
mkitem!{mkenum!{pub (crate) enum NetBlockingScalar { StdGetTtlUdp (u16) , StdSetTtlUdp (u16 , u32) , StdGetTtlTcp (u16) , StdSetTtlTcp (u16 , u32) , StdGetNodelay (u16) , StdSetNodelay (u16 , bool) , StdTcpClose (u16) , StdUdpClose (u16) , StdTcpStreamShutdown (u16 , crate :: net :: Shutdown) , }}}
mkitem!{mkenum!{pub (crate) enum NetLendMut { StdTcpConnect , StdTcpTx (u16) , StdTcpPeek (u16 , bool) , StdTcpRx (u16 , bool) , StdGetAddress (u16) , StdUdpBind , StdUdpRx (u16) , StdUdpTx (u16) , StdTcpListen , StdTcpAccept (u16) , }}}
mkitem!{mkimpl!{impl Into < usize > for NetLendMut { fn into (self) -> usize { match self { NetLendMut :: StdTcpConnect => 30 , NetLendMut :: StdTcpTx (fd) => 31 | ((fd as usize) << 16) , NetLendMut :: StdTcpPeek (fd , blocking) => { 32 | ((fd as usize) << 16) | if blocking { 0x8000 } else { 0 } } NetLendMut :: StdTcpRx (fd , blocking) => { 33 | ((fd as usize) << 16) | if blocking { 0x8000 } else { 0 } } NetLendMut :: StdGetAddress (fd) => 35 | ((fd as usize) << 16) , NetLendMut :: StdUdpBind => 40 , NetLendMut :: StdUdpRx (fd) => 42 | ((fd as usize) << 16) , NetLendMut :: StdUdpTx (fd) => 43 | ((fd as usize) << 16) , NetLendMut :: StdTcpListen => 44 , NetLendMut :: StdTcpAccept (fd) => 45 | ((fd as usize) << 16) , } } }}}
mkitem!{mkimpl!{impl < 'a > Into < [usize ; 5] > for NetBlockingScalar { fn into (self) -> [usize ; 5] { match self { NetBlockingScalar :: StdGetTtlTcp (fd) => [36 | ((fd as usize) << 16) , 0 , 0 , 0 , 0] , NetBlockingScalar :: StdGetTtlUdp (fd) => [36 | ((fd as usize) << 16) , 0 , 0 , 0 , 1] , NetBlockingScalar :: StdSetTtlTcp (fd , ttl) => { [37 | ((fd as usize) << 16) , ttl as _ , 0 , 0 , 0] } NetBlockingScalar :: StdSetTtlUdp (fd , ttl) => { [37 | ((fd as usize) << 16) , ttl as _ , 0 , 0 , 1] } NetBlockingScalar :: StdGetNodelay (fd) => [38 | ((fd as usize) << 16) , 0 , 0 , 0 , 0] , NetBlockingScalar :: StdSetNodelay (fd , enabled) => { [39 | ((fd as usize) << 16) , if enabled { 1 } else { 0 } , 0 , 0 , 1] } NetBlockingScalar :: StdTcpClose (fd) => [34 | ((fd as usize) << 16) , 0 , 0 , 0 , 0] , NetBlockingScalar :: StdUdpClose (fd) => [41 | ((fd as usize) << 16) , 0 , 0 , 0 , 0] , NetBlockingScalar :: StdTcpStreamShutdown (fd , how) => [46 | ((fd as usize) << 16) , match how { crate :: net :: Shutdown :: Read => 1 , crate :: net :: Shutdown :: Write => 2 , crate :: net :: Shutdown :: Both => 3 , } , 0 , 0 , 0 ,] , } } }}}

macro_rules! net_server_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function net_server in module {}", module_path!());
    };
}

mkfn!{
    net_server_introspect!();
    # [doc = " Returns a `Connection` to the Network server. This server provides all"] # [doc = " OS-level networking functions."] pub (crate) fn net_server () -> Connection { static NET_CONNECTION : Atomic < u32 > = AtomicU32 :: new (0) ; let cid = NET_CONNECTION . load (Ordering :: Relaxed) ; if cid != 0 { return cid . into () ; } let cid = connect ("_Middleware Network Server_") . unwrap () ; NET_CONNECTION . store (cid . into () , Ordering :: Relaxed) ; cid }
}