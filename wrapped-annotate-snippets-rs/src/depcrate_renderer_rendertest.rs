// Generated macro for test (module)
macro_rules! Depcrate_renderer_rendertest {
() => {
// Module: crate::renderer::render
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: { OUTPUT_REPLACEMENTS , newline_count } ; use snapbox :: IntoData ; fn format_replacements (replacements : Vec < (char , & str) >) -> String { replacements . into_iter () . map (| r | format ! ("    {r:?}")) . collect :: < Vec < _ > > () . join ("\n") } # [test] # [doc = " The [`OUTPUT_REPLACEMENTS`] array must be sorted (for binary search to"] # [doc = " work) and must contain no duplicate entries"] fn ensure_output_replacements_is_sorted () { let mut expected = OUTPUT_REPLACEMENTS . to_owned () ; expected . sort_by_key (| r | r . 0) ; expected . dedup_by_key (| r | r . 0) ; let expected = format_replacements (expected) ; let actual = format_replacements (OUTPUT_REPLACEMENTS . to_owned ()) ; snapbox :: assert_data_eq ! (actual , expected . into_data () . raw ()) ; } # [test] fn ensure_newline_count_correct () { let source = r#"
                cargo-features = ["path-bases"]

                [package]
                name = "foo"
                version = "0.5.0"
                authors = ["wycats@example.com"]

                [dependencies]
                bar = { base = '^^not-valid^^', path = 'bar' }
            "# ; let actual_count = newline_count (source) ; let expected_count = 10 ; assert_eq ! (expected_count , actual_count) ; } }
};
}
