mkuse!{use std :: fmt :: Write ;}
mkuse!{use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ;}
mkuse!{use rustc_hashes :: Hash64 ;}
mkuse!{use rustc_hir :: def_id :: CrateNum ;}
mkuse!{use rustc_middle :: ty :: { Instance , TyCtxt } ;}
mkuse!{use crate :: v0 ;}

macro_rules! mangle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mangle in module {}", module_path!());
    };
}

mkfn!{
    mangle_introspect!();
    pub (super) fn mangle < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , instantiating_crate : Option < CrateNum > , full_mangling_name : impl FnOnce () -> String ,) -> String { let crate_num = if let Some (krate) = instantiating_crate { krate } else { instance . def_id () . krate } ; let mut symbol = "_RNxC" . to_string () ; v0 :: push_ident (tcx . crate_name (crate_num) . as_str () , & mut symbol) ; let hash = tcx . with_stable_hashing_context (| mut hcx | { let mut hasher = StableHasher :: new () ; full_mangling_name () . hash_stable (& mut hcx , & mut hasher) ; hasher . finish :: < Hash64 > () . as_u64 () }) ; push_hash64 (hash , & mut symbol) ; symbol }
}

macro_rules! push_hash64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function push_hash64 in module {}", module_path!());
    };
}

mkfn!{
    push_hash64_introspect!();
    fn push_hash64 (hash : u64 , output : & mut String) { let hash = v0 :: encode_integer_62 (hash) ; let hash_len = hash . len () ; let _ = write ! (output , "{hash_len}H{}" , & hash [.. hash_len - 1]) ; }
}