macro_rules! detect_opaque_types_added_while_handling_opaque_types {
    () => {
        # [doc = " In theory `apply_concrete_opaque_types` could introduce new uses of opaque types."] # [doc = " We do not check these new uses so this could be unsound."] # [doc = ""] # [doc = " We detect any new uses and simply delay a bug if they occur. If this results in"] # [doc = " an ICE we can properly handle this, but we haven't encountered any such test yet."] # [doc = ""] # [doc = " See the related comment in `FnCtxt::detect_opaque_types_added_during_writeback`."] pub (crate) fn detect_opaque_types_added_while_handling_opaque_types < 'tcx > (infcx : & InferCtxt < 'tcx > , opaque_types_storage_num_entries : OpaqueTypeStorageEntries ,) { for (key , hidden_type) in infcx . inner . borrow_mut () . opaque_types () . opaque_types_added_since (opaque_types_storage_num_entries) { let opaque_type_string = infcx . tcx . def_path_str (key . def_id) ; let msg = format ! ("unexpected cyclic definition of `{opaque_type_string}`") ; infcx . dcx () . span_delayed_bug (hidden_type . span , msg) ; } let _ = infcx . take_opaque_types () ; }
    };
}

detect_opaque_types_added_while_handling_opaque_types!();