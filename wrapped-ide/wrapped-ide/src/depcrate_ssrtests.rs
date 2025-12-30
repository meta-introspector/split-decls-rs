// Generated macro for tests (module)
macro_rules! Depcrate_ssrtests {
() => {
// Module: crate::ssr
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use expect_test :: expect ; use ide_assists :: { Assist , AssistResolveStrategy } ; use ide_db :: { FileRange , FxHashSet , RootDatabase , base_db :: salsa :: Setter as _ , symbol_index :: LocalRoots , } ; use test_fixture :: WithFixture ; use super :: ssr_assists ; fn get_assists (# [rust_analyzer :: rust_fixture] ra_fixture : & str , resolve : AssistResolveStrategy ,) -> Vec < Assist > { let (mut db , file_id , range_or_offset) = RootDatabase :: with_range_or_offset (ra_fixture) ; let mut local_roots = FxHashSet :: default () ; local_roots . insert (test_fixture :: WORKSPACE) ; LocalRoots :: get (& db) . set_roots (& mut db) . to (local_roots) ; ssr_assists (& db , & resolve , FileRange { file_id : file_id . file_id (& db) , range : range_or_offset . into () } ,) } # [test] fn not_applicable_comment_not_ssr () { let ra_fixture = r#"
            //- /lib.rs

            // This is foo $0
            fn foo() {}
            "# ; let assists = get_assists (ra_fixture , AssistResolveStrategy :: All) ; assert_eq ! (0 , assists . len ()) ; } # [test] fn resolve_edits_true () { let assists = get_assists (r#"
            //- /lib.rs
            mod bar;

            // 2 ==>> 3$0
            fn foo() { 2 }

            //- /bar.rs
            fn bar() { 2 }
            "# , AssistResolveStrategy :: All ,) ; assert_eq ! (2 , assists . len ()) ; let mut assists = assists . into_iter () ; let apply_in_file_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "ssr",
                    RefactorRewrite,
                    None,
                ),
                label: "Apply SSR in file",
                group: Some(
                    GroupLabel(
                        "Apply SSR",
                    ),
                ),
                target: 10..21,
                source_change: Some(
                    SourceChange {
                        source_file_edits: {
                            FileId(
                                0,
                            ): (
                                TextEdit {
                                    indels: [
                                        Indel {
                                            insert: "3",
                                            delete: 33..34,
                                        },
                                    ],
                                    annotation: None,
                                },
                                None,
                            ),
                        },
                        file_system_edits: [],
                        is_snippet: false,
                        annotations: {},
                        next_annotation_id: 0,
                    },
                ),
                command: None,
            }
        "#]] . assert_debug_eq (& apply_in_file_assist) ; let apply_in_workspace_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "ssr",
                    RefactorRewrite,
                    None,
                ),
                label: "Apply SSR in workspace",
                group: Some(
                    GroupLabel(
                        "Apply SSR",
                    ),
                ),
                target: 10..21,
                source_change: Some(
                    SourceChange {
                        source_file_edits: {
                            FileId(
                                0,
                            ): (
                                TextEdit {
                                    indels: [
                                        Indel {
                                            insert: "3",
                                            delete: 33..34,
                                        },
                                    ],
                                    annotation: None,
                                },
                                None,
                            ),
                            FileId(
                                1,
                            ): (
                                TextEdit {
                                    indels: [
                                        Indel {
                                            insert: "3",
                                            delete: 11..12,
                                        },
                                    ],
                                    annotation: None,
                                },
                                None,
                            ),
                        },
                        file_system_edits: [],
                        is_snippet: false,
                        annotations: {},
                        next_annotation_id: 0,
                    },
                ),
                command: None,
            }
        "#]] . assert_debug_eq (& apply_in_workspace_assist) ; } # [test] fn resolve_edits_false () { let assists = get_assists (r#"
            //- /lib.rs
            mod bar;

            // 2 ==>> 3$0
            fn foo() { 2 }

            //- /bar.rs
            fn bar() { 2 }
            "# , AssistResolveStrategy :: None ,) ; assert_eq ! (2 , assists . len ()) ; let mut assists = assists . into_iter () ; let apply_in_file_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "ssr",
                    RefactorRewrite,
                    None,
                ),
                label: "Apply SSR in file",
                group: Some(
                    GroupLabel(
                        "Apply SSR",
                    ),
                ),
                target: 10..21,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& apply_in_file_assist) ; let apply_in_workspace_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "ssr",
                    RefactorRewrite,
                    None,
                ),
                label: "Apply SSR in workspace",
                group: Some(
                    GroupLabel(
                        "Apply SSR",
                    ),
                ),
                target: 10..21,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& apply_in_workspace_assist) ; } }
};
}
