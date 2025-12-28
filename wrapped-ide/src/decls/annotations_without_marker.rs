macro_rules! deps {
    () => {
        Analysis!();
        AnalysisHost!();
    };
}

macro_rules! annotations_without_marker {
    () => {
        deps!();
        # [doc = " Creates analysis from a multi-file fixture with annotations without $0"] pub (crate) fn annotations_without_marker (# [rust_analyzer :: rust_fixture] ra_fixture : & str ,) -> (Analysis , Vec < (FileRange , String) >) { let mut host = AnalysisHost :: default () ; let change_fixture = ChangeFixture :: parse (& host . db , ra_fixture) ; host . db . enable_proc_attr_macros () ; host . db . apply_change (change_fixture . change) ; let db = & host . db ; let annotations = change_fixture . files . iter () . flat_map (| & file_id | { let file_text = host . analysis () . file_text (file_id . file_id (db)) . unwrap () ; let annotations = extract_annotations (& file_text) ; annotations . into_iter () . map (move | (range , data) | (FileRange { file_id : file_id . file_id (db) , range } , data)) }) . collect () ; (host . analysis () , annotations) }
    };
}

annotations_without_marker!();