mkuse!{use core :: sync :: atomic :: { Atomic , AtomicU32 , Ordering } ;}
mkuse!{use crate :: os :: xous :: ffi :: Connection ;}
mkuse!{use crate :: os :: xous :: services :: connect ;}
mkitem!{mkenum!{# [repr (usize)] pub (crate) enum DnsLendMut { RawLookup = 6 , }}}
mkitem!{mkimpl!{impl Into < usize > for DnsLendMut { fn into (self) -> usize { self as usize } }}}

macro_rules! dns_server_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dns_server in module {}", module_path!());
    };
}

mkfn!{
    dns_server_introspect!();
    # [doc = " Returns a `Connection` to the DNS lookup server. This server is used for"] # [doc = " querying domain name values."] pub (crate) fn dns_server () -> Connection { static DNS_CONNECTION : Atomic < u32 > = AtomicU32 :: new (0) ; let cid = DNS_CONNECTION . load (Ordering :: Relaxed) ; if cid != 0 { return cid . into () ; } let cid = connect ("_DNS Resolver Middleware_") . unwrap () ; DNS_CONNECTION . store (cid . into () , Ordering :: Relaxed) ; cid }
}