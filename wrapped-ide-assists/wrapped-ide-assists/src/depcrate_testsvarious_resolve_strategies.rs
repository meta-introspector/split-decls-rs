// Generated macro for various_resolve_strategies (function)
macro_rules! Depcrate_testsvarious_resolve_strategies {
() => {
// Module: crate::tests
// Provides: {"various_resolve_strategies"}
// Dependencies: {}
# [test] fn various_resolve_strategies () { let (db , frange) = RootDatabase :: with_range (r#"
pub fn test_some_range(a: int) -> bool {
    if let 2..6 = $05$0 {
        true
    } else {
        false
    }
}
"# ,) ; let mut cfg = TEST_CONFIG ; cfg . allowed = Some (vec ! [AssistKind :: RefactorExtract]) ; { let assists = assists (& db , & cfg , AssistResolveStrategy :: None , FileRange { file_id : frange . file_id . file_id (& db) , range : frange . range } ,) ; assert_eq ! (4 , assists . len ()) ; let mut assists = assists . into_iter () ; let extract_into_variable_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_variable",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into variable",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_variable_assist) ; let extract_into_constant_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_constant",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into constant",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_constant_assist) ; let extract_into_static_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_static",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into static",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_static_assist) ; let extract_into_function_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_function",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into function",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_function_assist) ; } { let assists = assists (& db , & cfg , AssistResolveStrategy :: Single (SingleResolve { assist_id : "SOMETHING_MISMATCHING" . to_owned () , assist_kind : AssistKind :: RefactorExtract , assist_subtype : None , }) , FileRange { file_id : frange . file_id . file_id (& db) , range : frange . range } ,) ; assert_eq ! (4 , assists . len ()) ; let mut assists = assists . into_iter () ; let extract_into_variable_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_variable",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into variable",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_variable_assist) ; let extract_into_constant_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_constant",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into constant",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_constant_assist) ; let extract_into_static_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_static",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into static",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_static_assist) ; let extract_into_function_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_function",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into function",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_function_assist) ; } { let assists = assists (& db , & cfg , AssistResolveStrategy :: Single (SingleResolve { assist_id : "extract_variable" . to_owned () , assist_kind : AssistKind :: RefactorExtract , assist_subtype : None , }) , FileRange { file_id : frange . file_id . file_id (& db) , range : frange . range } ,) ; assert_eq ! (4 , assists . len ()) ; let mut assists = assists . into_iter () ; let extract_into_variable_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_variable",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into variable",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: Some(
                    SourceChange {
                        source_file_edits: {
                            FileId(
                                0,
                            ): (
                                TextEdit {
                                    indels: [
                                        Indel {
                                            insert: "let",
                                            delete: 45..47,
                                        },
                                        Indel {
                                            insert: "var_name",
                                            delete: 48..60,
                                        },
                                        Indel {
                                            insert: "=",
                                            delete: 61..81,
                                        },
                                        Indel {
                                            insert: "5;\n    if let 2..6 = var_name {\n        true\n    } else {\n        false\n    }",
                                            delete: 82..108,
                                        },
                                    ],
                                    annotation: None,
                                },
                                Some(
                                    SnippetEdit(
                                        [
                                            (
                                                0,
                                                49..49,
                                            ),
                                        ],
                                    ),
                                ),
                            ),
                        },
                        file_system_edits: [],
                        is_snippet: true,
                        annotations: {},
                        next_annotation_id: 0,
                    },
                ),
                command: Some(
                    Rename,
                ),
            }
        "#]] . assert_debug_eq (& extract_into_variable_assist) ; let extract_into_constant_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_constant",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into constant",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_constant_assist) ; let extract_into_static_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_static",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into static",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_static_assist) ; let extract_into_function_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_function",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into function",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: None,
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_function_assist) ; } { let assists = assists (& db , & cfg , AssistResolveStrategy :: All , FileRange { file_id : frange . file_id . file_id (& db) , range : frange . range } ,) ; assert_eq ! (4 , assists . len ()) ; let mut assists = assists . into_iter () ; let extract_into_variable_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_variable",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into variable",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: Some(
                    SourceChange {
                        source_file_edits: {
                            FileId(
                                0,
                            ): (
                                TextEdit {
                                    indels: [
                                        Indel {
                                            insert: "let",
                                            delete: 45..47,
                                        },
                                        Indel {
                                            insert: "var_name",
                                            delete: 48..60,
                                        },
                                        Indel {
                                            insert: "=",
                                            delete: 61..81,
                                        },
                                        Indel {
                                            insert: "5;\n    if let 2..6 = var_name {\n        true\n    } else {\n        false\n    }",
                                            delete: 82..108,
                                        },
                                    ],
                                    annotation: None,
                                },
                                Some(
                                    SnippetEdit(
                                        [
                                            (
                                                0,
                                                49..49,
                                            ),
                                        ],
                                    ),
                                ),
                            ),
                        },
                        file_system_edits: [],
                        is_snippet: true,
                        annotations: {},
                        next_annotation_id: 0,
                    },
                ),
                command: Some(
                    Rename,
                ),
            }
        "#]] . assert_debug_eq (& extract_into_variable_assist) ; let extract_into_constant_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_constant",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into constant",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: Some(
                    SourceChange {
                        source_file_edits: {
                            FileId(
                                0,
                            ): (
                                TextEdit {
                                    indels: [
                                        Indel {
                                            insert: "const",
                                            delete: 45..47,
                                        },
                                        Indel {
                                            insert: "VAR_NAME:",
                                            delete: 48..60,
                                        },
                                        Indel {
                                            insert: "i32",
                                            delete: 61..81,
                                        },
                                        Indel {
                                            insert: "=",
                                            delete: 82..86,
                                        },
                                        Indel {
                                            insert: "5;\n    if let 2..6 = VAR_NAME {\n        true\n    } else {\n        false\n    }",
                                            delete: 87..108,
                                        },
                                    ],
                                    annotation: None,
                                },
                                Some(
                                    SnippetEdit(
                                        [
                                            (
                                                0,
                                                51..51,
                                            ),
                                        ],
                                    ),
                                ),
                            ),
                        },
                        file_system_edits: [],
                        is_snippet: true,
                        annotations: {},
                        next_annotation_id: 0,
                    },
                ),
                command: Some(
                    Rename,
                ),
            }
        "#]] . assert_debug_eq (& extract_into_constant_assist) ; let extract_into_static_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_static",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into static",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: Some(
                    SourceChange {
                        source_file_edits: {
                            FileId(
                                0,
                            ): (
                                TextEdit {
                                    indels: [
                                        Indel {
                                            insert: "static",
                                            delete: 45..47,
                                        },
                                        Indel {
                                            insert: "VAR_NAME:",
                                            delete: 48..60,
                                        },
                                        Indel {
                                            insert: "i32",
                                            delete: 61..81,
                                        },
                                        Indel {
                                            insert: "=",
                                            delete: 82..86,
                                        },
                                        Indel {
                                            insert: "5;\n    if let 2..6 = VAR_NAME {\n        true\n    } else {\n        false\n    }",
                                            delete: 87..108,
                                        },
                                    ],
                                    annotation: None,
                                },
                                Some(
                                    SnippetEdit(
                                        [
                                            (
                                                0,
                                                52..52,
                                            ),
                                        ],
                                    ),
                                ),
                            ),
                        },
                        file_system_edits: [],
                        is_snippet: true,
                        annotations: {},
                        next_annotation_id: 0,
                    },
                ),
                command: Some(
                    Rename,
                ),
            }
        "#]] . assert_debug_eq (& extract_into_static_assist) ; let extract_into_function_assist = assists . next () . unwrap () ; expect ! [[r#"
            Assist {
                id: AssistId(
                    "extract_function",
                    RefactorExtract,
                    None,
                ),
                label: "Extract into function",
                group: Some(
                    GroupLabel(
                        "Extract into...",
                    ),
                ),
                target: 59..60,
                source_change: Some(
                    SourceChange {
                        source_file_edits: {
                            FileId(
                                0,
                            ): (
                                TextEdit {
                                    indels: [
                                        Indel {
                                            insert: "fun_name()",
                                            delete: 59..60,
                                        },
                                        Indel {
                                            insert: "\n\nfn fun_name() -> i32 {\n    5\n}",
                                            delete: 110..110,
                                        },
                                    ],
                                    annotation: None,
                                },
                                Some(
                                    SnippetEdit(
                                        [
                                            (
                                                0,
                                                124..124,
                                            ),
                                        ],
                                    ),
                                ),
                            ),
                        },
                        file_system_edits: [],
                        is_snippet: true,
                        annotations: {},
                        next_annotation_id: 0,
                    },
                ),
                command: None,
            }
        "#]] . assert_debug_eq (& extract_into_function_assist) ; } }
};
}
