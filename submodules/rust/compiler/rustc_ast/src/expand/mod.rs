mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable_Generic } ;}
mkmod!{allocator, { 
                getname!(allocator);
                getsrc!(allocator);
                getpath!(allocator);
                get_deps!(allocator);
                get_crates!(allocator);
                mkinclude!(allocator);
                 
            }}
mkmod!{autodiff_attrs, { 
                getname!(autodiff_attrs);
                getsrc!(autodiff_attrs);
                getpath!(autodiff_attrs);
                get_deps!(autodiff_attrs);
                get_crates!(autodiff_attrs);
                mkinclude!(autodiff_attrs);
                 
            }}
mkmod!{typetree, { 
                getname!(typetree);
                getsrc!(typetree);
                getpath!(typetree);
                get_deps!(typetree);
                get_crates!(typetree);
                mkinclude!(typetree);
                 
            }}