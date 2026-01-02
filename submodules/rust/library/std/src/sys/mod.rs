mkmod!{configure_builtins, { 
                getname!(configure_builtins);
                getsrc!(configure_builtins);
                getpath!(configure_builtins);
                get_deps!(configure_builtins);
                get_crates!(configure_builtins);
                mkinclude!(configure_builtins);
                 
            }}
mkmod!{pal, { 
                getname!(pal);
                getsrc!(pal);
                getpath!(pal);
                get_deps!(pal);
                get_crates!(pal);
                mkinclude!(pal);
                 
            }}
mkmod!{alloc, { 
                getname!(alloc);
                getsrc!(alloc);
                getpath!(alloc);
                get_deps!(alloc);
                get_crates!(alloc);
                mkinclude!(alloc);
                 
            }}
mkmod!{personality, { 
                getname!(personality);
                getsrc!(personality);
                getpath!(personality);
                get_deps!(personality);
                get_crates!(personality);
                mkinclude!(personality);
                 
            }}
mkmod!{anonymous_pipe, { 
                getname!(anonymous_pipe);
                getsrc!(anonymous_pipe);
                getpath!(anonymous_pipe);
                get_deps!(anonymous_pipe);
                get_crates!(anonymous_pipe);
                mkinclude!(anonymous_pipe);
                 
            }}
mkmod!{args, { 
                getname!(args);
                getsrc!(args);
                getpath!(args);
                get_deps!(args);
                get_crates!(args);
                mkinclude!(args);
                 
            }}
mkmod!{backtrace, { 
                getname!(backtrace);
                getsrc!(backtrace);
                getpath!(backtrace);
                get_deps!(backtrace);
                get_crates!(backtrace);
                mkinclude!(backtrace);
                 
            }}
mkmod!{cmath, { 
                getname!(cmath);
                getsrc!(cmath);
                getpath!(cmath);
                get_deps!(cmath);
                get_crates!(cmath);
                mkinclude!(cmath);
                 
            }}
mkmod!{env, { 
                getname!(env);
                getsrc!(env);
                getpath!(env);
                get_deps!(env);
                get_crates!(env);
                mkinclude!(env);
                 
            }}
mkmod!{env_consts, { 
                getname!(env_consts);
                getsrc!(env_consts);
                getpath!(env_consts);
                get_deps!(env_consts);
                get_crates!(env_consts);
                mkinclude!(env_consts);
                 
            }}
mkmod!{exit_guard, { 
                getname!(exit_guard);
                getsrc!(exit_guard);
                getpath!(exit_guard);
                get_deps!(exit_guard);
                get_crates!(exit_guard);
                mkinclude!(exit_guard);
                 
            }}
mkmod!{fd, { 
                getname!(fd);
                getsrc!(fd);
                getpath!(fd);
                get_deps!(fd);
                get_crates!(fd);
                mkinclude!(fd);
                 
            }}
mkmod!{fs, { 
                getname!(fs);
                getsrc!(fs);
                getpath!(fs);
                get_deps!(fs);
                get_crates!(fs);
                mkinclude!(fs);
                 
            }}
mkmod!{io, { 
                getname!(io);
                getsrc!(io);
                getpath!(io);
                get_deps!(io);
                get_crates!(io);
                mkinclude!(io);
                 
            }}
mkmod!{net, { 
                getname!(net);
                getsrc!(net);
                getpath!(net);
                get_deps!(net);
                get_crates!(net);
                mkinclude!(net);
                 
            }}
mkmod!{os_str, { 
                getname!(os_str);
                getsrc!(os_str);
                getpath!(os_str);
                get_deps!(os_str);
                get_crates!(os_str);
                mkinclude!(os_str);
                 
            }}
mkmod!{path, { 
                getname!(path);
                getsrc!(path);
                getpath!(path);
                get_deps!(path);
                get_crates!(path);
                mkinclude!(path);
                 
            }}
mkmod!{platform_version, { 
                getname!(platform_version);
                getsrc!(platform_version);
                getpath!(platform_version);
                get_deps!(platform_version);
                get_crates!(platform_version);
                mkinclude!(platform_version);
                 
            }}
mkmod!{process, { 
                getname!(process);
                getsrc!(process);
                getpath!(process);
                get_deps!(process);
                get_crates!(process);
                mkinclude!(process);
                 
            }}
mkmod!{random, { 
                getname!(random);
                getsrc!(random);
                getpath!(random);
                get_deps!(random);
                get_crates!(random);
                mkinclude!(random);
                 
            }}
mkmod!{stdio, { 
                getname!(stdio);
                getsrc!(stdio);
                getpath!(stdio);
                get_deps!(stdio);
                get_crates!(stdio);
                mkinclude!(stdio);
                 
            }}
mkmod!{sync, { 
                getname!(sync);
                getsrc!(sync);
                getpath!(sync);
                get_deps!(sync);
                get_crates!(sync);
                mkinclude!(sync);
                 
            }}
mkmod!{thread, { 
                getname!(thread);
                getsrc!(thread);
                getpath!(thread);
                get_deps!(thread);
                get_crates!(thread);
                mkinclude!(thread);
                 
            }}
mkmod!{thread_local, { 
                getname!(thread_local);
                getsrc!(thread_local);
                getpath!(thread_local);
                get_deps!(thread_local);
                get_crates!(thread_local);
                mkinclude!(thread_local);
                 
            }}
mkuse!{pub use pal :: * ;}