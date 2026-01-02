mkitem!{# [cfg (test)] extern crate self as rustc_serialize ;}
mkuse!{pub use self :: serialize :: { Decodable , Decoder , Encodable , Encoder } ;}
mkmod!{serialize, { 
                getname!(serialize);
                getsrc!(serialize);
                getpath!(serialize);
                get_deps!(serialize);
                get_crates!(serialize);
                mkinclude!(serialize);
                 
            }}
mkmod!{int_overflow, { 
                getname!(int_overflow);
                getsrc!(int_overflow);
                getpath!(int_overflow);
                get_deps!(int_overflow);
                get_crates!(int_overflow);
                mkinclude!(int_overflow);
                 
            }}
mkmod!{leb128, { 
                getname!(leb128);
                getsrc!(leb128);
                getpath!(leb128);
                get_deps!(leb128);
                get_crates!(leb128);
                mkinclude!(leb128);
                 
            }}
mkmod!{opaque, { 
                getname!(opaque);
                getsrc!(opaque);
                getpath!(opaque);
                get_deps!(opaque);
                get_crates!(opaque);
                mkinclude!(opaque);
                 
            }}