mkmod!{map, { 
                getname!(map);
                getsrc!(map);
                getpath!(map);
                get_deps!(map);
                get_crates!(map);
                mkinclude!(map);
                 
            }}
mkmod!{set, { 
                getname!(set);
                getsrc!(set);
                getpath!(set);
                get_deps!(set);
                get_crates!(set);
                mkinclude!(set);
                 
            }}
mkuse!{pub use map :: SsoHashMap ;}
mkuse!{pub use set :: SsoHashSet ;}