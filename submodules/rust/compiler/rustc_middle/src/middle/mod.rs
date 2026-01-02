mkmod!{codegen_fn_attrs, { 
                getname!(codegen_fn_attrs);
                getsrc!(codegen_fn_attrs);
                getpath!(codegen_fn_attrs);
                get_deps!(codegen_fn_attrs);
                get_crates!(codegen_fn_attrs);
                mkinclude!(codegen_fn_attrs);
                 
            }}
mkmod!{debugger_visualizer, { 
                getname!(debugger_visualizer);
                getsrc!(debugger_visualizer);
                getpath!(debugger_visualizer);
                get_deps!(debugger_visualizer);
                get_crates!(debugger_visualizer);
                mkinclude!(debugger_visualizer);
                 
            }}
mkmod!{dependency_format, { 
                getname!(dependency_format);
                getsrc!(dependency_format);
                getpath!(dependency_format);
                get_deps!(dependency_format);
                get_crates!(dependency_format);
                mkinclude!(dependency_format);
                 
            }}
mkmod!{exported_symbols, { 
                getname!(exported_symbols);
                getsrc!(exported_symbols);
                getpath!(exported_symbols);
                get_deps!(exported_symbols);
                get_crates!(exported_symbols);
                mkinclude!(exported_symbols);
                 
            }}
mkmod!{lang_items, { 
                getname!(lang_items);
                getsrc!(lang_items);
                getpath!(lang_items);
                get_deps!(lang_items);
                get_crates!(lang_items);
                mkinclude!(lang_items);
                 
            }}
mkmod!{lib_features, { 
                getname!(lib_features);
                getsrc!(lib_features);
                getpath!(lib_features);
                get_deps!(lib_features);
                get_crates!(lib_features);
                mkinclude!(lib_features);
                mkuse!{use rustc_data_structures :: unord :: UnordMap ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use rustc_span :: { Span , Symbol } ;}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , PartialEq , Eq)] # [derive (HashStable , TyEncodable , TyDecodable)] pub enum FeatureStability { AcceptedSince (Symbol) , Unstable { old_name : Option < Symbol > } , }}}
mkitem!{mkstruct!{# [derive (HashStable , Debug , Default)] pub struct LibFeatures { pub stability : UnordMap < Symbol , (FeatureStability , Span) > , }}}
mkitem!{mkimpl!{impl LibFeatures { pub fn to_sorted_vec (& self) -> Vec < (Symbol , FeatureStability) > { self . stability . to_sorted_stable_ord () . iter () . map (| & (& sym , & (stab , _)) | (sym , stab)) . collect () } }}} 
            }}
mkmod!{privacy, { 
                getname!(privacy);
                getsrc!(privacy);
                getpath!(privacy);
                get_deps!(privacy);
                get_crates!(privacy);
                mkinclude!(privacy);
                 
            }}
mkmod!{region, { 
                getname!(region);
                getsrc!(region);
                getpath!(region);
                get_deps!(region);
                get_crates!(region);
                mkinclude!(region);
                 
            }}
mkmod!{resolve_bound_vars, { 
                getname!(resolve_bound_vars);
                getsrc!(resolve_bound_vars);
                getpath!(resolve_bound_vars);
                get_deps!(resolve_bound_vars);
                get_crates!(resolve_bound_vars);
                mkinclude!(resolve_bound_vars);
                 
            }}
mkmod!{stability, { 
                getname!(stability);
                getsrc!(stability);
                getpath!(stability);
                get_deps!(stability);
                get_crates!(stability);
                mkinclude!(stability);
                 
            }}