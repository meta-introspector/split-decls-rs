mkuse!{use std :: hash :: BuildHasherDefault ;}
mkuse!{pub use ena :: unify :: { NoError , UnifyKey , UnifyValue } ;}
mkuse!{use rustc_hash :: FxHasher ;}
mkuse!{pub use rustc_hash :: { FxHashMap as HashMap , FxHashSet as HashSet } ;}
mkitem!{pub type IndexMap < K , V > = indexmap :: IndexMap < K , V , BuildHasherDefault < FxHasher > > ;}
mkitem!{pub type IndexSet < V > = indexmap :: IndexSet < V , BuildHasherDefault < FxHasher > > ;}
mkmod!{delayed_map, { 
                getname!(delayed_map);
                getsrc!(delayed_map);
                getpath!(delayed_map);
                get_deps!(delayed_map);
                get_crates!(delayed_map);
                mkinclude!(delayed_map);
                 
            }}
mkmod!{impl_, { 
                getname!(impl_);
                getsrc!(impl_);
                getpath!(impl_);
                get_deps!(impl_);
                get_crates!(impl_);
                mkinclude!(impl_);
                mkuse!{pub use rustc_data_structures :: sso :: { SsoHashMap , SsoHashSet } ;}
mkuse!{pub use rustc_data_structures :: stack :: ensure_sufficient_stack ;} 
            }}
mkmod!{impl_, { 
                getname!(impl_);
                getsrc!(impl_);
                getpath!(impl_);
                get_deps!(impl_);
                get_crates!(impl_);
                mkinclude!(impl_);
                mkuse!{pub use std :: collections :: { HashMap as SsoHashMap , HashSet as SsoHashSet } ;}

macro_rules! ensure_sufficient_stack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ensure_sufficient_stack in module {}", module_path!());
    };
}

mkfn!{
    ensure_sufficient_stack_introspect!();
    # [inline] pub fn ensure_sufficient_stack < R > (f : impl FnOnce () -> R) -> R { f () }
} 
            }}
mkuse!{pub use delayed_map :: { DelayedMap , DelayedSet } ;}
mkuse!{pub use impl_ :: * ;}