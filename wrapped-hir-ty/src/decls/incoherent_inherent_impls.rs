macro_rules! deps {
    () => {
        HirDatabase!();
        InherentImpls!();
    };
}

macro_rules! incoherent_inherent_impls {
    () => {
        deps!();
        pub fn incoherent_inherent_impls (db : & dyn HirDatabase , self_ty : SimplifiedType) -> & [ImplId] { let has_incoherent_impls = match self_ty . def () { Some (def_id) => match def_id . try_into () { Ok (def_id) => { db . attrs (def_id) . by_key (sym :: rustc_has_incoherent_inherent_impls) . exists () } Err (()) => true , } , _ => true , } ; return if ! has_incoherent_impls { & [] } else { incoherent_inherent_impls_query (db , () , self_ty) } ; # [salsa :: tracked (returns (ref))] fn incoherent_inherent_impls_query (db : & dyn HirDatabase , _force_query_input_to_be_interned : () , self_ty : SimplifiedType ,) -> Box < [ImplId] > { let _p = tracing :: info_span ! ("incoherent_inherent_impl_crates") . entered () ; let mut result = Vec :: new () ; for & krate in crates_containing_incoherent_inherent_impls (db) { let impls = InherentImpls :: for_crate (db , krate) ; result . extend_from_slice (impls . for_self_ty (& self_ty)) ; } result . into_boxed_slice () } }
    };
}

incoherent_inherent_impls!()