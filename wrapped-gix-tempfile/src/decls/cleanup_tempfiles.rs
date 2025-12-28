macro_rules! cleanup_tempfiles {
    () => {
        # [doc = " Remove all tempfiles still registered on our global registry."] # [doc = " This happens on a best-effort basis with all errors being ignored."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " Must not be called from within signal hooks. For that, use [`cleanup_tempfiles_signal_safe()`]."] pub fn cleanup_tempfiles () { let current_pid = std :: process :: id () ; # [cfg (feature = "hp-hashmap")] REGISTRY . iter_mut () . for_each (| mut tf | { if tf . as_ref () . is_some_and (| tf | tf . owning_process_id == current_pid) { tf . take () ; } }) ; # [cfg (not (feature = "hp-hashmap"))] REGISTRY . for_each (| tf | { if tf . as_ref () . map_or (false , | tf | tf . owning_process_id == current_pid) { tf . take () ; } }) ; }
    };
}

cleanup_tempfiles!()