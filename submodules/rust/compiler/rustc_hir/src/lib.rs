mkitem!{extern crate self as rustc_hir ;}
mkmod!{arena, { 
                getname!(arena);
                getsrc!(arena);
                getpath!(arena);
                get_deps!(arena);
                get_crates!(arena);
                mkinclude!(arena);
                 
            }}
mkmod!{attrs, { 
                getname!(attrs);
                getsrc!(attrs);
                getpath!(attrs);
                get_deps!(attrs);
                get_crates!(attrs);
                mkinclude!(attrs);
                 
            }}
mkmod!{def, { 
                getname!(def);
                getsrc!(def);
                getpath!(def);
                get_deps!(def);
                get_crates!(def);
                mkinclude!(def);
                 
            }}
mkmod!{def_path_hash_map, { 
                getname!(def_path_hash_map);
                getsrc!(def_path_hash_map);
                getpath!(def_path_hash_map);
                get_deps!(def_path_hash_map);
                get_crates!(def_path_hash_map);
                mkinclude!(def_path_hash_map);
                 
            }}
mkmod!{definitions, { 
                getname!(definitions);
                getsrc!(definitions);
                getpath!(definitions);
                get_deps!(definitions);
                get_crates!(definitions);
                mkinclude!(definitions);
                 
            }}
mkmod!{diagnostic_items, { 
                getname!(diagnostic_items);
                getsrc!(diagnostic_items);
                getpath!(diagnostic_items);
                get_deps!(diagnostic_items);
                get_crates!(diagnostic_items);
                mkinclude!(diagnostic_items);
                 
            }}
mkuse!{pub use rustc_span :: def_id ;}
mkmod!{hir, { 
                getname!(hir);
                getsrc!(hir);
                getpath!(hir);
                get_deps!(hir);
                get_crates!(hir);
                mkinclude!(hir);
                 
            }}
mkuse!{pub use rustc_hir_id :: { self as hir_id , * } ;}
mkmod!{intravisit, { 
                getname!(intravisit);
                getsrc!(intravisit);
                getpath!(intravisit);
                get_deps!(intravisit);
                get_crates!(intravisit);
                mkinclude!(intravisit);
                 
            }}
mkmod!{lang_items, { 
                getname!(lang_items);
                getsrc!(lang_items);
                getpath!(lang_items);
                get_deps!(lang_items);
                get_crates!(lang_items);
                mkinclude!(lang_items);
                 
            }}
mkmod!{limit, { 
                getname!(limit);
                getsrc!(limit);
                getpath!(limit);
                get_deps!(limit);
                get_crates!(limit);
                mkinclude!(limit);
                 
            }}
mkmod!{lints, { 
                getname!(lints);
                getsrc!(lints);
                getpath!(lints);
                get_deps!(lints);
                get_crates!(lints);
                mkinclude!(lints);
                 
            }}
mkmod!{pat_util, { 
                getname!(pat_util);
                getsrc!(pat_util);
                getpath!(pat_util);
                get_deps!(pat_util);
                get_crates!(pat_util);
                mkinclude!(pat_util);
                 
            }}
mkmod!{stability, { 
                getname!(stability);
                getsrc!(stability);
                getpath!(stability);
                get_deps!(stability);
                get_crates!(stability);
                mkinclude!(stability);
                 
            }}
mkmod!{stable_hash_impls, { 
                getname!(stable_hash_impls);
                getsrc!(stable_hash_impls);
                getpath!(stable_hash_impls);
                get_deps!(stable_hash_impls);
                get_crates!(stable_hash_impls);
                mkinclude!(stable_hash_impls);
                 
            }}
mkmod!{target, { 
                getname!(target);
                getsrc!(target);
                getpath!(target);
                get_deps!(target);
                get_crates!(target);
                mkinclude!(target);
                 
            }}
mkmod!{version, { 
                getname!(version);
                getsrc!(version);
                getpath!(version);
                get_deps!(version);
                get_crates!(version);
                mkinclude!(version);
                 
            }}
mkmod!{weak_lang_items, { 
                getname!(weak_lang_items);
                getsrc!(weak_lang_items);
                getpath!(weak_lang_items);
                get_deps!(weak_lang_items);
                get_crates!(weak_lang_items);
                mkinclude!(weak_lang_items);
                 
            }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{# [doc (no_inline)] pub use hir :: * ;}
mkuse!{pub use lang_items :: { LangItem , LanguageItems } ;}
mkuse!{pub use stability :: * ;}
mkuse!{pub use stable_hash_impls :: HashStableContext ;}
mkuse!{pub use target :: { MethodKind , Target } ;}
mkuse!{pub use version :: * ;}
mkitem!{arena_types ! (rustc_arena :: declare_arena) ;}