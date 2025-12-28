macro_rules! deps {
    () => {
        Diagnostic!();
        DiagnosticsConfig!();
    };
}

macro_rules! check_diagnostics_with_config {
    () => {
        deps!();
        # [track_caller] pub (crate) fn check_diagnostics_with_config (config : DiagnosticsConfig , # [rust_analyzer :: rust_fixture] ra_fixture : & str ,) { let _tracing = setup_tracing () ; let (db , files) = RootDatabase :: with_many_files (ra_fixture) ; let mut annotations = files . iter () . copied () . flat_map (| file_id | { hir :: attach_db (& db , | | { super :: full_diagnostics (& db , & config , & AssistResolveStrategy :: All , file_id . file_id (& db) ,) . into_iter () . map (| d | { let mut annotation = String :: new () ; if let Some (fixes) = & d . fixes { assert ! (! fixes . is_empty ()) ; annotation . push_str ("💡 ") } annotation . push_str (match d . severity { Severity :: Error => "error" , Severity :: WeakWarning => "weak" , Severity :: Warning => "warn" , Severity :: Allow => "allow" , }) ; annotation . push_str (": ") ; annotation . push_str (& d . message) ; (d . range , annotation) }) }) }) . map (| (diagnostic , annotation) | (diagnostic . file_id , (diagnostic . range , annotation))) . into_group_map () ; for file_id in files { let file_id = file_id . file_id (& db) ; let line_index = db . line_index (file_id) ; let mut actual = annotations . remove (& file_id) . unwrap_or_default () ; let mut expected = extract_annotations (db . file_text (file_id) . text (& db)) ; expected . sort_by_key (| (range , s) | (range . start () , s . clone ())) ; actual . sort_by_key (| (range , s) | (range . start () , s . clone ())) ; actual . dedup () ; if expected . is_empty () { for (e , _) in & actual { eprintln ! ("Code in range {e:?} = {}" , & db . file_text (file_id) . text (& db) [usize :: from (e . start ()) .. usize :: from (e . end ())]) } } if expected != actual { let fneg = expected . iter () . filter (| x | ! actual . contains (x)) . map (| (range , s) | (line_index . line_col (range . start ()) , range , s)) . collect :: < Vec < _ > > () ; let fpos = actual . iter () . filter (| x | ! expected . contains (x)) . map (| (range , s) | (line_index . line_col (range . start ()) , range , s)) . collect :: < Vec < _ > > () ; panic ! ("Diagnostic test failed.\nFalse negatives: {fneg:?}\nFalse positives: {fpos:?}") ; } } }
    };
}

check_diagnostics_with_config!();