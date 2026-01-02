mkuse!{use std :: fmt ;}
mkuse!{pub use atomic_ref :: AtomicRef ;}
mkuse!{pub use ena :: { snapshot_vec , undo_log , unify } ;}
mkuse!{pub use rustc_index :: static_assert_size ;}
mkmod!{aligned, { 
                getname!(aligned);
                getsrc!(aligned);
                getpath!(aligned);
                get_deps!(aligned);
                get_crates!(aligned);
                mkinclude!(aligned);
                 
            }}
mkmod!{base_n, { 
                getname!(base_n);
                getsrc!(base_n);
                getpath!(base_n);
                get_deps!(base_n);
                get_crates!(base_n);
                mkinclude!(base_n);
                 
            }}
mkmod!{binary_search_util, { 
                getname!(binary_search_util);
                getsrc!(binary_search_util);
                getpath!(binary_search_util);
                get_deps!(binary_search_util);
                get_crates!(binary_search_util);
                mkinclude!(binary_search_util);
                 
            }}
mkmod!{fingerprint, { 
                getname!(fingerprint);
                getsrc!(fingerprint);
                getpath!(fingerprint);
                get_deps!(fingerprint);
                get_crates!(fingerprint);
                mkinclude!(fingerprint);
                 
            }}
mkmod!{flat_map_in_place, { 
                getname!(flat_map_in_place);
                getsrc!(flat_map_in_place);
                getpath!(flat_map_in_place);
                get_deps!(flat_map_in_place);
                get_crates!(flat_map_in_place);
                mkinclude!(flat_map_in_place);
                 
            }}
mkmod!{flock, { 
                getname!(flock);
                getsrc!(flock);
                getpath!(flock);
                get_deps!(flock);
                get_crates!(flock);
                mkinclude!(flock);
                 
            }}
mkmod!{frozen, { 
                getname!(frozen);
                getsrc!(frozen);
                getpath!(frozen);
                get_deps!(frozen);
                get_crates!(frozen);
                mkinclude!(frozen);
                 
            }}
mkmod!{fx, { 
                getname!(fx);
                getsrc!(fx);
                getpath!(fx);
                get_deps!(fx);
                get_crates!(fx);
                mkinclude!(fx);
                 
            }}
mkmod!{graph, { 
                getname!(graph);
                getsrc!(graph);
                getpath!(graph);
                get_deps!(graph);
                get_crates!(graph);
                mkinclude!(graph);
                 
            }}
mkmod!{intern, { 
                getname!(intern);
                getsrc!(intern);
                getpath!(intern);
                get_deps!(intern);
                get_crates!(intern);
                mkinclude!(intern);
                 
            }}
mkmod!{jobserver, { 
                getname!(jobserver);
                getsrc!(jobserver);
                getpath!(jobserver);
                get_deps!(jobserver);
                get_crates!(jobserver);
                mkinclude!(jobserver);
                 
            }}
mkmod!{marker, { 
                getname!(marker);
                getsrc!(marker);
                getpath!(marker);
                get_deps!(marker);
                get_crates!(marker);
                mkinclude!(marker);
                 
            }}
mkmod!{memmap, { 
                getname!(memmap);
                getsrc!(memmap);
                getpath!(memmap);
                get_deps!(memmap);
                get_crates!(memmap);
                mkinclude!(memmap);
                 
            }}
mkmod!{obligation_forest, { 
                getname!(obligation_forest);
                getsrc!(obligation_forest);
                getpath!(obligation_forest);
                get_deps!(obligation_forest);
                get_crates!(obligation_forest);
                mkinclude!(obligation_forest);
                 
            }}
mkmod!{owned_slice, { 
                getname!(owned_slice);
                getsrc!(owned_slice);
                getpath!(owned_slice);
                get_deps!(owned_slice);
                get_crates!(owned_slice);
                mkinclude!(owned_slice);
                 
            }}
mkmod!{packed, { 
                getname!(packed);
                getsrc!(packed);
                getpath!(packed);
                get_deps!(packed);
                get_crates!(packed);
                mkinclude!(packed);
                 
            }}
mkmod!{profiling, { 
                getname!(profiling);
                getsrc!(profiling);
                getpath!(profiling);
                get_deps!(profiling);
                get_crates!(profiling);
                mkinclude!(profiling);
                 
            }}
mkmod!{sharded, { 
                getname!(sharded);
                getsrc!(sharded);
                getpath!(sharded);
                get_deps!(sharded);
                get_crates!(sharded);
                mkinclude!(sharded);
                 
            }}
mkmod!{small_c_str, { 
                getname!(small_c_str);
                getsrc!(small_c_str);
                getpath!(small_c_str);
                get_deps!(small_c_str);
                get_crates!(small_c_str);
                mkinclude!(small_c_str);
                 
            }}
mkmod!{snapshot_map, { 
                getname!(snapshot_map);
                getsrc!(snapshot_map);
                getpath!(snapshot_map);
                get_deps!(snapshot_map);
                get_crates!(snapshot_map);
                mkinclude!(snapshot_map);
                 
            }}
mkmod!{sorted_map, { 
                getname!(sorted_map);
                getsrc!(sorted_map);
                getpath!(sorted_map);
                get_deps!(sorted_map);
                get_crates!(sorted_map);
                mkinclude!(sorted_map);
                 
            }}
mkmod!{sso, { 
                getname!(sso);
                getsrc!(sso);
                getpath!(sso);
                get_deps!(sso);
                get_crates!(sso);
                mkinclude!(sso);
                 
            }}
mkmod!{stable_hasher, { 
                getname!(stable_hasher);
                getsrc!(stable_hasher);
                getpath!(stable_hasher);
                get_deps!(stable_hasher);
                get_crates!(stable_hasher);
                mkinclude!(stable_hasher);
                 
            }}
mkmod!{stack, { 
                getname!(stack);
                getsrc!(stack);
                getpath!(stack);
                get_deps!(stack);
                get_crates!(stack);
                mkinclude!(stack);
                 
            }}
mkmod!{steal, { 
                getname!(steal);
                getsrc!(steal);
                getpath!(steal);
                get_deps!(steal);
                get_crates!(steal);
                mkinclude!(steal);
                 
            }}
mkmod!{svh, { 
                getname!(svh);
                getsrc!(svh);
                getpath!(svh);
                get_deps!(svh);
                get_crates!(svh);
                mkinclude!(svh);
                 
            }}
mkmod!{sync, { 
                getname!(sync);
                getsrc!(sync);
                getpath!(sync);
                get_deps!(sync);
                get_crates!(sync);
                mkinclude!(sync);
                 
            }}
mkmod!{tagged_ptr, { 
                getname!(tagged_ptr);
                getsrc!(tagged_ptr);
                getpath!(tagged_ptr);
                get_deps!(tagged_ptr);
                get_crates!(tagged_ptr);
                mkinclude!(tagged_ptr);
                 
            }}
mkmod!{temp_dir, { 
                getname!(temp_dir);
                getsrc!(temp_dir);
                getpath!(temp_dir);
                get_deps!(temp_dir);
                get_crates!(temp_dir);
                mkinclude!(temp_dir);
                 
            }}
mkmod!{thinvec, { 
                getname!(thinvec);
                getsrc!(thinvec);
                getpath!(thinvec);
                get_deps!(thinvec);
                get_crates!(thinvec);
                mkinclude!(thinvec);
                 
            }}
mkmod!{thousands, { 
                getname!(thousands);
                getsrc!(thousands);
                getpath!(thousands);
                get_deps!(thousands);
                get_crates!(thousands);
                mkinclude!(thousands);
                 
            }}
mkmod!{transitive_relation, { 
                getname!(transitive_relation);
                getsrc!(transitive_relation);
                getpath!(transitive_relation);
                get_deps!(transitive_relation);
                get_crates!(transitive_relation);
                mkinclude!(transitive_relation);
                 
            }}
mkmod!{unhash, { 
                getname!(unhash);
                getsrc!(unhash);
                getpath!(unhash);
                get_deps!(unhash);
                get_crates!(unhash);
                mkinclude!(unhash);
                 
            }}
mkmod!{union_find, { 
                getname!(union_find);
                getsrc!(union_find);
                getpath!(union_find);
                get_deps!(union_find);
                get_crates!(union_find);
                mkinclude!(union_find);
                 
            }}
mkmod!{unord, { 
                getname!(unord);
                getsrc!(unord);
                getpath!(unord);
                get_deps!(unord);
                get_crates!(unord);
                mkinclude!(unord);
                 
            }}
mkmod!{vec_cache, { 
                getname!(vec_cache);
                getsrc!(vec_cache);
                getpath!(vec_cache);
                get_deps!(vec_cache);
                get_crates!(vec_cache);
                mkinclude!(vec_cache);
                 
            }}
mkmod!{work_queue, { 
                getname!(work_queue);
                getsrc!(work_queue);
                getpath!(work_queue);
                get_deps!(work_queue);
                get_crates!(work_queue);
                mkinclude!(work_queue);
                 
            }}
mkmod!{atomic_ref, { 
                getname!(atomic_ref);
                getsrc!(atomic_ref);
                getpath!(atomic_ref);
                get_deps!(atomic_ref);
                get_crates!(atomic_ref);
                mkinclude!(atomic_ref);
                 
            }}

macro_rules! outline_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function outline in module {}", module_path!());
    };
}

mkfn!{
    outline_introspect!();
    # [doc = " This calls the passed function while ensuring it won't be inlined into the caller."] # [inline (never)] # [cold] pub fn outline < F : FnOnce () -> R , R > (f : F) -> R { f () }
}

macro_rules! defer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function defer in module {}", module_path!());
    };
}

mkfn!{
    defer_introspect!();
    # [doc = " Returns a structure that calls `f` when dropped."] pub fn defer < F : FnOnce () > (f : F) -> OnDrop < F > { OnDrop (Some (f)) }
}
mkitem!{mkstruct!{pub struct OnDrop < F : FnOnce () > (Option < F >) ;}}
mkitem!{mkimpl!{impl < F : FnOnce () > OnDrop < F > { # [doc = " Disables on-drop call."] # [inline] pub fn disable (mut self) { self . 0 . take () ; } }}}
mkitem!{mkimpl!{impl < F : FnOnce () > Drop for OnDrop < F > { # [inline] fn drop (& mut self) { if let Some (f) = self . 0 . take () { f () ; } } }}}
mkitem!{mkstruct!{# [doc = " This is a marker for a fatal compiler error used with `resume_unwind`."] pub struct FatalErrorMarker ;}}

macro_rules! make_display_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_display in module {}", module_path!());
    };
}

mkfn!{
    make_display_introspect!();
    # [doc = " Turns a closure that takes an `&mut Formatter` into something that can be display-formatted."] pub fn make_display (f : impl Fn (& mut fmt :: Formatter < '_ >) -> fmt :: Result) -> impl fmt :: Display { struct Printer < F > { f : F , } impl < F > fmt :: Display for Printer < F > where F : Fn (& mut fmt :: Formatter < '_ >) -> fmt :: Result , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (self . f) (fmt) } } Printer { f } }
}

macro_rules! __noop_fix_for_windows_dllimport_issue_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function __noop_fix_for_windows_dllimport_issue in module {}", module_path!());
    };
}

mkfn!{
    __noop_fix_for_windows_dllimport_issue_introspect!();
    # [doc (hidden)] pub fn __noop_fix_for_windows_dllimport_issue () { }
}
mkitem!{# [macro_export] macro_rules ! external_bitflags_debug { ($ Name : ident) => { impl :: std :: fmt :: Debug for $ Name { fn fmt (& self , f : & mut :: std :: fmt :: Formatter <'_ >) -> :: std :: fmt :: Result { :: bitflags :: parser :: to_writer (self , f) } } } ; }}