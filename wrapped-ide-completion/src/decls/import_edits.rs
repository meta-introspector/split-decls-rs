macro_rules! deps {
    () => {
        CompletionContext!();
    };
}

macro_rules! import_edits {
    () => {
        deps!();
        fn import_edits (ctx : & CompletionContext < '_ > , requires : & [ModPath]) -> Option < Vec < LocatedImport > > { let import_cfg = ctx . config . find_path_config (ctx . is_nightly) ; let resolve = | import | { let item = ctx . scope . resolve_mod_path (import) . next () ? ; let path = ctx . module . find_use_path (ctx . db , item , ctx . config . insert_use . prefix_kind , import_cfg ,) ? ; Some ((path . len () > 1) . then (| | LocatedImport :: new_no_completion (path . clone () , item , item))) } ; let mut res = Vec :: with_capacity (requires . len ()) ; for import in requires { match resolve (import) { Some (first) => res . extend (first) , None => return None , } } Some (res) }
    };
}

import_edits!()