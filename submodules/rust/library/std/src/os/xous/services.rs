mkuse!{use core :: sync :: atomic :: { Atomic , AtomicU32 , Ordering } ;}
mkuse!{use crate :: os :: xous :: ffi :: Connection ;}
mkmod!{dns, { 
                getname!(dns);
                getsrc!(dns);
                getpath!(dns);
                get_deps!(dns);
                get_crates!(dns);
                mkinclude!(dns);
                 
            }}
mkuse!{pub (crate) use dns :: * ;}
mkmod!{log, { 
                getname!(log);
                getsrc!(log);
                getpath!(log);
                get_deps!(log);
                get_crates!(log);
                mkinclude!(log);
                 
            }}
mkuse!{pub (crate) use log :: * ;}
mkmod!{net, { 
                getname!(net);
                getsrc!(net);
                getpath!(net);
                get_deps!(net);
                get_crates!(net);
                mkinclude!(net);
                 
            }}
mkuse!{pub (crate) use net :: * ;}
mkmod!{systime, { 
                getname!(systime);
                getsrc!(systime);
                getpath!(systime);
                get_deps!(systime);
                get_crates!(systime);
                mkinclude!(systime);
                 
            }}
mkuse!{pub (crate) use systime :: * ;}
mkmod!{ticktimer, { 
                getname!(ticktimer);
                getsrc!(ticktimer);
                getpath!(ticktimer);
                get_deps!(ticktimer);
                get_crates!(ticktimer);
                mkinclude!(ticktimer);
                 
            }}
mkuse!{pub (crate) use ticktimer :: * ;}
mkmod!{ns, { 
                getname!(ns);
                getsrc!(ns);
                getpath!(ns);
                get_deps!(ns);
                get_crates!(ns);
                mkinclude!(ns);
                mkitem!{const NAME_MAX_LENGTH : usize = 64 ;}
mkuse!{use crate :: os :: xous :: ffi :: { Connection , lend_mut } ;}
mkitem!{mkstruct!{# [repr (C , align (4096))] struct ConnectRequest { data : [u8 ; 4096] , }}}
mkitem!{mkimpl!{impl ConnectRequest { pub fn new (name : & str) -> Self { let mut cr = ConnectRequest { data : [0u8 ; 4096] } ; let name_bytes = name . as_bytes () ; for (& src_byte , dest_byte) in name_bytes . iter () . zip (& mut cr . data [0 .. NAME_MAX_LENGTH]) { * dest_byte = src_byte ; } for (& src_byte , dest_byte) in (name . len () . min (NAME_MAX_LENGTH) as u32) . to_le_bytes () . iter () . zip (& mut cr . data [NAME_MAX_LENGTH ..]) { * dest_byte = src_byte ; } cr } }}}

macro_rules! connect_with_name_impl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function connect_with_name_impl in module {}", module_path!());
    };
}

mkfn!{
    connect_with_name_impl_introspect!();
    pub fn connect_with_name_impl (name : & str , blocking : bool) -> Option < Connection > { let mut request = ConnectRequest :: new (name) ; let opcode = if blocking { 6 } else { 7 } ; let cid = if blocking { super :: name_server () } else { super :: try_name_server () ? } ; lend_mut (cid , opcode , & mut request . data , 0 , name . len () . min (NAME_MAX_LENGTH)) . expect ("unable to perform lookup") ; let result = u32 :: from_le_bytes (request . data [0 .. 4] . try_into () . unwrap ()) ; if result == 0 { Some (u32 :: from_le_bytes (request . data [4 .. 8] . try_into () . unwrap ()) . into ()) } else { None } }
}

macro_rules! connect_with_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function connect_with_name in module {}", module_path!());
    };
}

mkfn!{
    connect_with_name_introspect!();
    pub fn connect_with_name (name : & str) -> Option < Connection > { connect_with_name_impl (name , true) }
}

macro_rules! try_connect_with_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_connect_with_name in module {}", module_path!());
    };
}

mkfn!{
    try_connect_with_name_introspect!();
    pub fn try_connect_with_name (name : & str) -> Option < Connection > { connect_with_name_impl (name , false) }
} 
            }}

macro_rules! connect_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function connect in module {}", module_path!());
    };
}

mkfn!{
    connect_introspect!();
    # [doc = " Attempts to connect to a server by name. If the server does not exist, this will"] # [doc = " block until the server is created."] # [doc = ""] # [doc = " Note that this is different from connecting to a server by address. Server"] # [doc = " addresses are always 16 bytes long, whereas server names are arbitrary-length"] # [doc = " strings up to 64 bytes in length."] # [stable (feature = "rust1" , since = "1.0.0")] pub fn connect (name : & str) -> Option < Connection > { ns :: connect_with_name (name) }
}

macro_rules! try_connect_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_connect in module {}", module_path!());
    };
}

mkfn!{
    try_connect_introspect!();
    # [doc = " Attempts to connect to a server by name. If the server does not exist, this will"] # [doc = " immediately return `None`."] # [doc = ""] # [doc = " Note that this is different from connecting to a server by address. Server"] # [doc = " addresses are always 16 bytes long, whereas server names are arbitrary-length"] # [doc = " strings."] # [stable (feature = "rust1" , since = "1.0.0")] pub fn try_connect (name : & str) -> Option < Connection > { ns :: try_connect_with_name (name) }
}
mkitem!{static NAME_SERVER_CONNECTION : Atomic < u32 > = AtomicU32 :: new (0) ;}

macro_rules! name_server_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function name_server in module {}", module_path!());
    };
}

mkfn!{
    name_server_introspect!();
    # [doc = " Returns a `Connection` to the name server. If the name server has not been started,"] # [doc = " then this call will block until the name server has been started. The `Connection`"] # [doc = " will be shared among all connections in a process, so it is safe to call this"] # [doc = " multiple times."] pub (crate) fn name_server () -> Connection { let cid = NAME_SERVER_CONNECTION . load (Ordering :: Relaxed) ; if cid != 0 { return cid . into () ; } let cid = crate :: os :: xous :: ffi :: connect ("xous-name-server" . try_into () . unwrap ()) . unwrap () ; NAME_SERVER_CONNECTION . store (cid . into () , Ordering :: Relaxed) ; cid }
}

macro_rules! try_name_server_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_name_server in module {}", module_path!());
    };
}

mkfn!{
    try_name_server_introspect!();
    fn try_name_server () -> Option < Connection > { let cid = NAME_SERVER_CONNECTION . load (Ordering :: Relaxed) ; if cid != 0 { return Some (cid . into ()) ; } if let Ok (Some (cid)) = crate :: os :: xous :: ffi :: try_connect ("xous-name-server" . try_into () . unwrap ()) { NAME_SERVER_CONNECTION . store (cid . into () , Ordering :: Relaxed) ; Some (cid) } else { None } }
}