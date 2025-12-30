// Generated macro for tests (module)
macro_rules! Depcrate_annotationstests {
() => {
// Module: crate::annotations
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use expect_test :: { Expect , expect } ; use ide_db :: MiniCore ; use crate :: { Annotation , AnnotationConfig , fixture } ; use super :: AnnotationLocation ; const DEFAULT_CONFIG : AnnotationConfig < '_ > = AnnotationConfig { binary_target : true , annotate_runnables : true , annotate_impls : true , annotate_references : true , annotate_method_references : true , annotate_enum_variant_references : true , location : AnnotationLocation :: AboveName , minicore : MiniCore :: default () , filter_adjacent_derive_implementations : false , } ; fn check_with_config (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect , config : & AnnotationConfig < '_ > ,) { let (analysis , file_id) = fixture :: file (ra_fixture) ; let annotations : Vec < Annotation > = analysis . annotations (config , file_id) . unwrap () . into_iter () . map (| annotation | analysis . resolve_annotation (& DEFAULT_CONFIG , annotation) . unwrap ()) . collect () ; expect . assert_debug_eq (& annotations) ; } fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { check_with_config (ra_fixture , expect , & DEFAULT_CONFIG) ; } # [test] fn const_annotations () { check (r#"
const DEMO: i32 = 123;

const UNUSED: i32 = 123;

fn main() {
    let hello = DEMO;
}
            "# , expect ! [[r#"
                [
                    Annotation {
                        range: 6..10,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 6,
                            },
                            data: Some(
                                [
                                    FileRangeWrapper {
                                        file_id: FileId(
                                            0,
                                        ),
                                        range: 78..82,
                                    },
                                ],
                            ),
                        },
                    },
                    Annotation {
                        range: 30..36,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 30,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                    Annotation {
                        range: 53..57,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 53,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                    Annotation {
                        range: 53..57,
                        kind: Runnable(
                            Runnable {
                                use_name_in_title: false,
                                nav: NavigationTarget {
                                    file_id: FileId(
                                        0,
                                    ),
                                    full_range: 50..85,
                                    focus_range: 53..57,
                                    name: "main",
                                    kind: Function,
                                },
                                kind: Bin,
                                cfg: None,
                                update_test: UpdateTest {
                                    expect_test: false,
                                    insta: false,
                                    snapbox: false,
                                },
                            },
                        ),
                    },
                ]
            "#]] ,) ; } # [test] fn struct_references_annotations () { check (r#"
struct Test;

fn main() {
    let test = Test;
}
            "# , expect ! [[r#"
                [
                    Annotation {
                        range: 7..11,
                        kind: HasImpls {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 7,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                    Annotation {
                        range: 7..11,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 7,
                            },
                            data: Some(
                                [
                                    FileRangeWrapper {
                                        file_id: FileId(
                                            0,
                                        ),
                                        range: 41..45,
                                    },
                                ],
                            ),
                        },
                    },
                    Annotation {
                        range: 17..21,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 17,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                    Annotation {
                        range: 17..21,
                        kind: Runnable(
                            Runnable {
                                use_name_in_title: false,
                                nav: NavigationTarget {
                                    file_id: FileId(
                                        0,
                                    ),
                                    full_range: 14..48,
                                    focus_range: 17..21,
                                    name: "main",
                                    kind: Function,
                                },
                                kind: Bin,
                                cfg: None,
                                update_test: UpdateTest {
                                    expect_test: false,
                                    insta: false,
                                    snapbox: false,
                                },
                            },
                        ),
                    },
                ]
            "#]] ,) ; } # [test] fn struct_and_trait_impls_annotations () { check (r#"
struct Test;

trait MyCoolTrait {}

impl MyCoolTrait for Test {}

fn main() {
    let test = Test;
}
            "# , expect ! [[r#"
                [
                    Annotation {
                        range: 7..11,
                        kind: HasImpls {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 7,
                            },
                            data: Some(
                                [
                                    NavigationTarget {
                                        file_id: FileId(
                                            0,
                                        ),
                                        full_range: 36..64,
                                        focus_range: 57..61,
                                        name: "impl",
                                        kind: Impl,
                                    },
                                ],
                            ),
                        },
                    },
                    Annotation {
                        range: 7..11,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 7,
                            },
                            data: Some(
                                [
                                    FileRangeWrapper {
                                        file_id: FileId(
                                            0,
                                        ),
                                        range: 57..61,
                                    },
                                    FileRangeWrapper {
                                        file_id: FileId(
                                            0,
                                        ),
                                        range: 93..97,
                                    },
                                ],
                            ),
                        },
                    },
                    Annotation {
                        range: 20..31,
                        kind: HasImpls {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 20,
                            },
                            data: Some(
                                [
                                    NavigationTarget {
                                        file_id: FileId(
                                            0,
                                        ),
                                        full_range: 36..64,
                                        focus_range: 57..61,
                                        name: "impl",
                                        kind: Impl,
                                    },
                                ],
                            ),
                        },
                    },
                    Annotation {
                        range: 20..31,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 20,
                            },
                            data: Some(
                                [
                                    FileRangeWrapper {
                                        file_id: FileId(
                                            0,
                                        ),
                                        range: 41..52,
                                    },
                                ],
                            ),
                        },
                    },
                    Annotation {
                        range: 69..73,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 69,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                    Annotation {
                        range: 69..73,
                        kind: Runnable(
                            Runnable {
                                use_name_in_title: false,
                                nav: NavigationTarget {
                                    file_id: FileId(
                                        0,
                                    ),
                                    full_range: 66..100,
                                    focus_range: 69..73,
                                    name: "main",
                                    kind: Function,
                                },
                                kind: Bin,
                                cfg: None,
                                update_test: UpdateTest {
                                    expect_test: false,
                                    insta: false,
                                    snapbox: false,
                                },
                            },
                        ),
                    },
                ]
            "#]] ,) ; } # [test] fn runnable_annotation () { check (r#"
fn main() {}
            "# , expect ! [[r#"
                [
                    Annotation {
                        range: 3..7,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 3,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                    Annotation {
                        range: 3..7,
                        kind: Runnable(
                            Runnable {
                                use_name_in_title: false,
                                nav: NavigationTarget {
                                    file_id: FileId(
                                        0,
                                    ),
                                    full_range: 0..12,
                                    focus_range: 3..7,
                                    name: "main",
                                    kind: Function,
                                },
                                kind: Bin,
                                cfg: None,
                                update_test: UpdateTest {
                                    expect_test: false,
                                    insta: false,
                                    snapbox: false,
                                },
                            },
                        ),
                    },
                ]
            "#]] ,) ; } # [test] fn method_annotations () { check (r#"
struct Test;

impl Test {
    fn self_by_ref(&self) {}
}

fn main() {
    Test.self_by_ref();
}
            "# , expect ! [[r#"
                [
                    Annotation {
                        range: 7..11,
                        kind: HasImpls {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 7,
                            },
                            data: Some(
                                [
                                    NavigationTarget {
                                        file_id: FileId(
                                            0,
                                        ),
                                        full_range: 14..56,
                                        focus_range: 19..23,
                                        name: "impl",
                                        kind: Impl,
                                    },
                                ],
                            ),
                        },
                    },
                    Annotation {
                        range: 7..11,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 7,
                            },
                            data: Some(
                                [
                                    FileRangeWrapper {
                                        file_id: FileId(
                                            0,
                                        ),
                                        range: 19..23,
                                    },
                                    FileRangeWrapper {
                                        file_id: FileId(
                                            0,
                                        ),
                                        range: 74..78,
                                    },
                                ],
                            ),
                        },
                    },
                    Annotation {
                        range: 33..44,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 33,
                            },
                            data: Some(
                                [
                                    FileRangeWrapper {
                                        file_id: FileId(
                                            0,
                                        ),
                                        range: 79..90,
                                    },
                                ],
                            ),
                        },
                    },
                    Annotation {
                        range: 61..65,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 61,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                    Annotation {
                        range: 61..65,
                        kind: Runnable(
                            Runnable {
                                use_name_in_title: false,
                                nav: NavigationTarget {
                                    file_id: FileId(
                                        0,
                                    ),
                                    full_range: 58..95,
                                    focus_range: 61..65,
                                    name: "main",
                                    kind: Function,
                                },
                                kind: Bin,
                                cfg: None,
                                update_test: UpdateTest {
                                    expect_test: false,
                                    insta: false,
                                    snapbox: false,
                                },
                            },
                        ),
                    },
                ]
            "#]] ,) ; } # [test] fn test_annotations () { check (r#"
fn main() {}

mod tests {
    #[test]
    fn my_cool_test() {}
}
            "# , expect ! [[r#"
                [
                    Annotation {
                        range: 3..7,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 3,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                    Annotation {
                        range: 3..7,
                        kind: Runnable(
                            Runnable {
                                use_name_in_title: false,
                                nav: NavigationTarget {
                                    file_id: FileId(
                                        0,
                                    ),
                                    full_range: 0..12,
                                    focus_range: 3..7,
                                    name: "main",
                                    kind: Function,
                                },
                                kind: Bin,
                                cfg: None,
                                update_test: UpdateTest {
                                    expect_test: false,
                                    insta: false,
                                    snapbox: false,
                                },
                            },
                        ),
                    },
                    Annotation {
                        range: 18..23,
                        kind: Runnable(
                            Runnable {
                                use_name_in_title: false,
                                nav: NavigationTarget {
                                    file_id: FileId(
                                        0,
                                    ),
                                    full_range: 14..64,
                                    focus_range: 18..23,
                                    name: "tests",
                                    kind: Module,
                                    description: "mod tests",
                                },
                                kind: TestMod {
                                    path: "tests",
                                },
                                cfg: None,
                                update_test: UpdateTest {
                                    expect_test: false,
                                    insta: false,
                                    snapbox: false,
                                },
                            },
                        ),
                    },
                    Annotation {
                        range: 45..57,
                        kind: Runnable(
                            Runnable {
                                use_name_in_title: false,
                                nav: NavigationTarget {
                                    file_id: FileId(
                                        0,
                                    ),
                                    full_range: 30..62,
                                    focus_range: 45..57,
                                    name: "my_cool_test",
                                    kind: Function,
                                },
                                kind: Test {
                                    test_id: Path(
                                        "tests::my_cool_test",
                                    ),
                                    attr: TestAttr {
                                        ignore: false,
                                    },
                                },
                                cfg: None,
                                update_test: UpdateTest {
                                    expect_test: false,
                                    insta: false,
                                    snapbox: false,
                                },
                            },
                        ),
                    },
                ]
            "#]] ,) ; } # [test] fn test_no_annotations_outside_module_tree () { check (r#"
//- /foo.rs
struct Foo;
//- /lib.rs
// this file comes last since `check` checks the first file only
"# , expect ! [[r#"
                []
            "#]] ,) ; } # [test] fn test_no_annotations_macro_struct_def () { check (r#"
//- /lib.rs
macro_rules! m {
    () => {
        struct A {}
    };
}

m!();
"# , expect ! [[r#"
                []
            "#]] ,) ; } # [test] fn test_annotations_macro_struct_def_call_site () { check (r#"
//- /lib.rs
macro_rules! m {
    ($name:ident) => {
        struct $name {}
    };
}

m! {
    Name
};
"# , expect ! [[r#"
                [
                    Annotation {
                        range: 83..87,
                        kind: HasImpls {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 83,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                    Annotation {
                        range: 83..87,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 83,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                ]
            "#]] ,) ; } # [test] fn test_annotations_appear_above_whole_item_when_configured_to_do_so () { check_with_config (r#"
/// This is a struct named Foo, obviously.
#[derive(Clone)]
struct Foo;
"# , expect ! [[r#"
                [
                    Annotation {
                        range: 0..71,
                        kind: HasImpls {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 67,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                    Annotation {
                        range: 0..71,
                        kind: HasReferences {
                            pos: FilePositionWrapper {
                                file_id: FileId(
                                    0,
                                ),
                                offset: 67,
                            },
                            data: Some(
                                [],
                            ),
                        },
                    },
                ]
            "#]] , & AnnotationConfig { location : AnnotationLocation :: AboveWholeItem , .. DEFAULT_CONFIG } ,) ; } }
};
}
