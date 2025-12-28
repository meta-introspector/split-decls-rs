macro_rules! owns_allocation {
    () => {
        fn owns_allocation (tcx : TyCtxt < '_ > , ty : Ty < '_ >) -> bool { if ty . is_array () { true } else if let Some (inner) = ty . boxed_ty () { inner . is_slice () || inner . is_str () || inner . ty_adt_def () . is_some_and (| def | tcx . is_lang_item (def . did () , LangItem :: CStr)) || owns_allocation (tcx , inner) } else if let Some (def) = ty . ty_adt_def () { for lang_item in [LangItem :: String , LangItem :: MaybeUninit , LangItem :: UnsafeCell] { if tcx . is_lang_item (def . did () , lang_item) { return true ; } } tcx . get_diagnostic_name (def . did ()) . is_some_and (| name | { matches ! (name , sym :: cstring_type | sym :: Vec | sym :: Cell | sym :: SyncUnsafeCell) }) } else { false } }
    };
}

owns_allocation!()