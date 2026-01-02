mkmod!{typeid, { 
                getname!(typeid);
                getsrc!(typeid);
                getpath!(typeid);
                get_deps!(typeid);
                get_crates!(typeid);
                mkinclude!(typeid);
                 
            }}
mkuse!{pub use crate :: cfi :: typeid :: { TypeIdOptions , typeid_for_fnabi , typeid_for_instance } ;}