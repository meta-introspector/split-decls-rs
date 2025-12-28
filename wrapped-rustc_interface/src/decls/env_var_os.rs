macro_rules! env_var_os {
    () => {
        fn env_var_os < 'tcx > (tcx : TyCtxt < 'tcx > , key : & 'tcx OsStr) -> Option < & 'tcx OsStr > { let value = env :: var_os (key) ; let value_tcx = value . as_ref () . map (| value | { let encoded_bytes = tcx . arena . alloc_slice (value . as_encoded_bytes ()) ; debug_assert_eq ! (value . as_encoded_bytes () , encoded_bytes) ; unsafe { OsStr :: from_encoded_bytes_unchecked (encoded_bytes) } }) ; tcx . sess . psess . env_depinfo . borrow_mut () . insert ((Symbol :: intern (& key . to_string_lossy ()) , value . as_ref () . and_then (| value | value . to_str ()) . map (| value | Symbol :: intern (& value)) ,)) ; value_tcx }
    };
}

env_var_os!();