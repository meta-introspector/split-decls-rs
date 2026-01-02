mkmod!{usercalls, { 
                getname!(usercalls);
                getsrc!(usercalls);
                getpath!(usercalls);
                get_deps!(usercalls);
                get_crates!(usercalls);
                mkinclude!(usercalls);
                mkuse!{pub use crate :: sys :: abi :: usercalls :: * ;}
mkmod!{alloc, { 
                getname!(alloc);
                getsrc!(alloc);
                getpath!(alloc);
                get_deps!(alloc);
                get_crates!(alloc);
                mkinclude!(alloc);
                mkuse!{pub use crate :: sys :: abi :: usercalls :: alloc :: * ;} 
            }}
mkmod!{raw, { 
                getname!(raw);
                getsrc!(raw);
                getpath!(raw);
                get_deps!(raw);
                get_crates!(raw);
                mkinclude!(raw);
                mkuse!{pub use crate :: sys :: abi :: usercalls :: raw :: { ByteBuffer , Cancel , EV_RETURNQ_NOT_EMPTY , EV_UNPARK , EV_USERCALLQ_NOT_FULL , Error , FD_STDERR , FD_STDIN , FD_STDOUT , Fd , FifoDescriptor , RESULT_SUCCESS , Register , RegisterArgument , Result , Return , ReturnValue , Tcs , USERCALL_USER_DEFINED , Usercall , Usercalls as UsercallNrs , WAIT_INDEFINITE , WAIT_NO , accept_stream , alloc , async_queues , bind_stream , close , connect_stream , do_usercall , exit , flush , free , insecure_time , launch_thread , read , read_alloc , send , wait , write , } ;} 
            }} 
            }}
mkmod!{mem, { 
                getname!(mem);
                getsrc!(mem);
                getpath!(mem);
                get_deps!(mem);
                get_crates!(mem);
                mkinclude!(mem);
                mkuse!{pub use crate :: sys :: abi :: mem :: * ;} 
            }}
mkmod!{arch, { 
                getname!(arch);
                getsrc!(arch);
                getpath!(arch);
                get_deps!(arch);
                get_crates!(arch);
                mkinclude!(arch);
                 
            }}
mkmod!{ffi, { 
                getname!(ffi);
                getsrc!(ffi);
                getpath!(ffi);
                get_deps!(ffi);
                get_crates!(ffi);
                mkinclude!(ffi);
                 
            }}
mkmod!{io, { 
                getname!(io);
                getsrc!(io);
                getpath!(io);
                get_deps!(io);
                get_crates!(io);
                mkinclude!(io);
                 
            }}
mkmod!{thread, { 
                getname!(thread);
                getsrc!(thread);
                getpath!(thread);
                get_deps!(thread);
                get_crates!(thread);
                mkinclude!(thread);
                mkuse!{pub use crate :: sys :: abi :: thread :: current ;} 
            }}