mkmod!{alloc, { 
                getname!(alloc);
                getsrc!(alloc);
                getpath!(alloc);
                get_deps!(alloc);
                get_crates!(alloc);
                mkinclude!(alloc);
                 
            }}
mkmod!{body, { 
                getname!(body);
                getsrc!(body);
                getpath!(body);
                get_deps!(body);
                get_crates!(body);
                mkinclude!(body);
                 
            }}
mkmod!{mono, { 
                getname!(mono);
                getsrc!(mono);
                getpath!(mono);
                get_deps!(mono);
                get_crates!(mono);
                mkinclude!(mono);
                 
            }}
mkmod!{pretty, { 
                getname!(pretty);
                getsrc!(pretty);
                getpath!(pretty);
                get_deps!(pretty);
                get_crates!(pretty);
                mkinclude!(pretty);
                 
            }}
mkmod!{visit, { 
                getname!(visit);
                getsrc!(visit);
                getpath!(visit);
                get_deps!(visit);
                get_crates!(visit);
                mkinclude!(visit);
                 
            }}
mkuse!{pub use body :: * ;}
mkuse!{pub use visit :: { MirVisitor , MutMirVisitor } ;}