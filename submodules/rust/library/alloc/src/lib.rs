mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{raw_vec, { 
                getname!(raw_vec);
                getsrc!(raw_vec);
                getpath!(raw_vec);
                get_deps!(raw_vec);
                get_crates!(raw_vec);
                mkinclude!(raw_vec);
                 
            }}
mkmod!{alloc, { 
                getname!(alloc);
                getsrc!(alloc);
                getpath!(alloc);
                get_deps!(alloc);
                get_crates!(alloc);
                mkinclude!(alloc);
                 
            }}
mkmod!{borrow, { 
                getname!(borrow);
                getsrc!(borrow);
                getpath!(borrow);
                get_deps!(borrow);
                get_crates!(borrow);
                mkinclude!(borrow);
                 
            }}
mkmod!{boxed, { 
                getname!(boxed);
                getsrc!(boxed);
                getpath!(boxed);
                get_deps!(boxed);
                get_crates!(boxed);
                mkinclude!(boxed);
                 
            }}
mkmod!{bstr, { 
                getname!(bstr);
                getsrc!(bstr);
                getpath!(bstr);
                get_deps!(bstr);
                get_crates!(bstr);
                mkinclude!(bstr);
                 
            }}
mkmod!{collections, { 
                getname!(collections);
                getsrc!(collections);
                getpath!(collections);
                get_deps!(collections);
                get_crates!(collections);
                mkinclude!(collections);
                 
            }}
mkmod!{ffi, { 
                getname!(ffi);
                getsrc!(ffi);
                getpath!(ffi);
                get_deps!(ffi);
                get_crates!(ffi);
                mkinclude!(ffi);
                 
            }}
mkmod!{fmt, { 
                getname!(fmt);
                getsrc!(fmt);
                getpath!(fmt);
                get_deps!(fmt);
                get_crates!(fmt);
                mkinclude!(fmt);
                 
            }}
mkmod!{rc, { 
                getname!(rc);
                getsrc!(rc);
                getpath!(rc);
                get_deps!(rc);
                get_crates!(rc);
                mkinclude!(rc);
                 
            }}
mkmod!{slice, { 
                getname!(slice);
                getsrc!(slice);
                getpath!(slice);
                get_deps!(slice);
                get_crates!(slice);
                mkinclude!(slice);
                 
            }}
mkmod!{str, { 
                getname!(str);
                getsrc!(str);
                getpath!(str);
                get_deps!(str);
                get_crates!(str);
                mkinclude!(str);
                 
            }}
mkmod!{string, { 
                getname!(string);
                getsrc!(string);
                getpath!(string);
                get_deps!(string);
                get_crates!(string);
                mkinclude!(string);
                 
            }}
mkmod!{sync, { 
                getname!(sync);
                getsrc!(sync);
                getpath!(sync);
                get_deps!(sync);
                get_crates!(sync);
                mkinclude!(sync);
                 
            }}
mkmod!{task, { 
                getname!(task);
                getsrc!(task);
                getpath!(task);
                get_deps!(task);
                get_crates!(task);
                mkinclude!(task);
                 
            }}
mkmod!{vec, { 
                getname!(vec);
                getsrc!(vec);
                getpath!(vec);
                get_deps!(vec);
                get_crates!(vec);
                mkinclude!(vec);
                 
            }}
mkmod!{wtf8, { 
                getname!(wtf8);
                getsrc!(wtf8);
                getpath!(wtf8);
                get_deps!(wtf8);
                get_crates!(wtf8);
                mkinclude!(wtf8);
                 
            }}
mkmod!{__export, { 
                getname!(__export);
                getsrc!(__export);
                getpath!(__export);
                get_deps!(__export);
                get_crates!(__export);
                mkinclude!(__export);
                mkuse!{pub use core :: format_args ;}
mkuse!{pub use core :: hint :: must_use ;} 
            }}