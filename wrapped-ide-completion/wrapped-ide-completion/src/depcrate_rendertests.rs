// Generated macro for tests (module)
macro_rules! Depcrate_rendertests {
() => {
// Module: crate::render
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: cmp ; use expect_test :: { Expect , expect } ; use ide_db :: SymbolKind ; use itertools :: Itertools ; use crate :: { CompletionItem , CompletionItemKind , CompletionRelevance , CompletionRelevancePostfixMatch , item :: CompletionRelevanceTypeMatch , tests :: { TEST_CONFIG , check_edit , do_completion , get_all_items } , } ; # [track_caller] fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str , kind : impl Into < CompletionItemKind > , expect : Expect ,) { let actual = do_completion (ra_fixture , kind . into ()) ; expect . assert_debug_eq (& actual) ; } # [track_caller] fn check_kinds (# [rust_analyzer :: rust_fixture] ra_fixture : & str , kinds : & [CompletionItemKind] , expect : Expect ,) { let actual : Vec < _ > = kinds . iter () . flat_map (| & kind | do_completion (ra_fixture , kind)) . collect () ; expect . assert_debug_eq (& actual) ; } # [track_caller] fn check_function_relevance (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let actual : Vec < _ > = do_completion (ra_fixture , CompletionItemKind :: SymbolKind (SymbolKind :: Method)) . into_iter () . map (| item | (item . detail . unwrap_or_default () , item . relevance . function)) . collect () ; expect . assert_debug_eq (& actual) ; } # [track_caller] fn check_relevance_for_kinds (# [rust_analyzer :: rust_fixture] ra_fixture : & str , kinds : & [CompletionItemKind] , expect : Expect ,) { let mut actual = get_all_items (TEST_CONFIG , ra_fixture , None) ; actual . retain (| it | kinds . contains (& it . kind)) ; actual . sort_by_key (| it | (cmp :: Reverse (it . relevance . score ()) , it . label . primary . clone ())) ; check_relevance_ (actual , expect) ; } # [track_caller] fn check_relevance (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let mut actual = get_all_items (TEST_CONFIG , ra_fixture , None) ; actual . retain (| it | it . kind != CompletionItemKind :: Snippet) ; actual . retain (| it | it . kind != CompletionItemKind :: Keyword) ; actual . retain (| it | it . kind != CompletionItemKind :: BuiltinType) ; actual . sort_by_key (| it | (cmp :: Reverse (it . relevance . score ()) , it . label . primary . clone ())) ; check_relevance_ (actual , expect) ; } # [track_caller] fn check_relevance_ (actual : Vec < CompletionItem > , expect : Expect) { let actual = actual . into_iter () . flat_map (| it | { let mut items = vec ! [] ; let tag = it . kind . tag () ; let relevance = display_relevance (it . relevance) ; items . push (format ! ("{tag} {} {} {relevance}\n" , it . label . primary , it . label . detail_right . clone () . unwrap_or_default () ,)) ; if let Some ((label , _indel , relevance)) = it . ref_match () { let relevance = display_relevance (relevance) ; items . push (format ! ("{tag} {label} {relevance}\n")) ; } items }) . collect :: < String > () ; expect . assert_eq (& actual) ; fn display_relevance (relevance : CompletionRelevance) -> String { let relevance_factors = vec ! [(relevance . type_match == Some (CompletionRelevanceTypeMatch :: Exact) , "type") , (relevance . type_match == Some (CompletionRelevanceTypeMatch :: CouldUnify) , "type_could_unify" ,) , (relevance . exact_name_match , "name") , (relevance . is_local , "local") , (relevance . postfix_match == Some (CompletionRelevancePostfixMatch :: Exact) , "snippet" ,) , (relevance . trait_ . is_some_and (| it | it . is_op_method) , "op_method") , (relevance . requires_import , "requires_import") ,] . into_iter () . filter_map (| (cond , desc) | if cond { Some (desc) } else { None }) . join ("+") ; format ! ("[{relevance_factors}]") } } # [test] fn set_struct_type_completion_info () { check_relevance (r#"
//- /lib.rs crate:dep

pub mod test_mod_b {
    pub struct Struct {}
}

pub mod test_mod_a {
    pub struct Struct {}
}

//- /main.rs crate:main deps:dep

fn test(input: dep::test_mod_b::Struct) { }

fn main() {
    test(Struct$0);
}
"# , expect ! [[r#"
                st dep::test_mod_b::Struct {…} dep::test_mod_b::Struct {  } [type_could_unify]
                ex dep::test_mod_b::Struct {  }  [type_could_unify]
                st Struct Struct [type_could_unify+requires_import]
                md dep  []
                fn main() fn() []
                fn test(…) fn(Struct) []
                st Struct Struct [requires_import]
            "#]] ,) ; } # [test] fn set_union_type_completion_info () { check_relevance (r#"
//- /lib.rs crate:dep

pub mod test_mod_b {
    pub union Union {
        a: i32,
        b: i32
    }
}

pub mod test_mod_a {
    pub enum Union {
        a: i32,
        b: i32
    }
}

//- /main.rs crate:main deps:dep

fn test(input: dep::test_mod_b::Union) { }

fn main() {
    test(Union$0);
}
"# , expect ! [[r#"
                un Union Union [type_could_unify+requires_import]
                md dep  []
                fn main() fn() []
                fn test(…) fn(Union) []
                en Union Union [requires_import]
            "#]] ,) ; } # [test] fn set_enum_type_completion_info () { check_relevance (r#"
//- /lib.rs crate:dep

pub mod test_mod_b {
    pub enum Enum {
        variant
    }
}

pub mod test_mod_a {
    pub enum Enum {
        variant
    }
}

//- /main.rs crate:main deps:dep

fn test(input: dep::test_mod_b::Enum) { }

fn main() {
    test(Enum$0);
}
"# , expect ! [[r#"
                ev dep::test_mod_b::Enum::variant dep::test_mod_b::Enum::variant [type_could_unify]
                ex dep::test_mod_b::Enum::variant  [type_could_unify]
                en Enum Enum [type_could_unify+requires_import]
                md dep  []
                fn main() fn() []
                fn test(…) fn(Enum) []
                en Enum Enum [requires_import]
            "#]] ,) ; } # [test] fn set_enum_variant_type_completion_info () { check_relevance (r#"
//- /lib.rs crate:dep

pub mod test_mod_b {
    pub enum Enum {
        Variant
    }
}

pub mod test_mod_a {
    pub enum Enum {
        Variant
    }
}

//- /main.rs crate:main deps:dep

fn test(input: dep::test_mod_b::Enum) { }

fn main() {
    test(Variant$0);
}
"# , expect ! [[r#"
                ev dep::test_mod_b::Enum::Variant dep::test_mod_b::Enum::Variant [type_could_unify]
                ex dep::test_mod_b::Enum::Variant  [type_could_unify]
                md dep  []
                fn main() fn() []
                fn test(…) fn(Enum) []
            "#]] ,) ; } # [test] fn set_fn_type_completion_info () { check_relevance (r#"
//- /lib.rs crate:dep

pub mod test_mod_b {
    pub fn function(j: isize) -> i32 {}
}

pub mod test_mod_a {
    pub fn function(i: usize) -> i32 {}
}

//- /main.rs crate:main deps:dep

fn test(input: fn(usize) -> i32) { }

fn main() {
    test(function$0);
}
"# , expect ! [[r#"
                md dep  []
                fn main() fn() []
                fn test(…) fn(fn(usize) -> i32) []
                fn function fn(usize) -> i32 [requires_import]
                fn function(…) fn(isize) -> i32 [requires_import]
            "#]] ,) ; } # [test] fn set_const_type_completion_info () { check_relevance (r#"
//- /lib.rs crate:dep

pub mod test_mod_b {
    pub const CONST: i32 = 1;
}

pub mod test_mod_a {
    pub const CONST: i64 = 2;
}

//- /main.rs crate:main deps:dep

fn test(input: i32) { }

fn main() {
    test(CONST$0);
}
"# , expect ! [[r#"
                ct CONST i32 [type_could_unify+requires_import]
                md dep  []
                fn main() fn() []
                fn test(…) fn(i32) []
                ct CONST i64 [requires_import]
            "#]] ,) ; } # [test] fn set_static_type_completion_info () { check_relevance (r#"
//- /lib.rs crate:dep

pub mod test_mod_b {
    pub static STATIC: i32 = 5;
}

pub mod test_mod_a {
    pub static STATIC: i64 = 5;
}

//- /main.rs crate:main deps:dep

fn test(input: i32) { }

fn main() {
    test(STATIC$0);
}
"# , expect ! [[r#"
                sc STATIC i32 [type_could_unify+requires_import]
                md dep  []
                fn main() fn() []
                fn test(…) fn(i32) []
                sc STATIC i64 [requires_import]
            "#]] ,) ; } # [test] fn set_self_type_completion_info_with_params () { check_relevance (r#"
//- /lib.rs crate:dep
pub struct Struct;

impl Struct {
    pub fn Function(&self, input: i32) -> bool {
                false
    }
}


//- /main.rs crate:main deps:dep

use dep::Struct;


fn test(input: fn(&dep::Struct, i32) -> bool) { }

fn main() {
    test(Struct::Function$0);
}

"# , expect ! [[r#"
                me Function fn(&self, i32) -> bool []
            "#]] ,) ; } # [test] fn set_self_type_completion_info () { check_relevance (r#"
//- /main.rs crate:main

struct Struct;

impl Struct {
fn test(&self) {
        func(Self$0);
    }
}

fn func(input: Struct) { }

"# , expect ! [[r#"
                st Self Self [type]
                st Struct Struct [type]
                sp Self Struct [type]
                st Struct Struct [type]
                ex Struct  [type]
                lc self &Struct [local]
                fn func(…) fn(Struct) []
                me self.test() fn(&self) []
            "#]] ,) ; } # [test] fn set_builtin_type_completion_info () { check_relevance (r#"
//- /main.rs crate:main

fn test(input: bool) { }
    pub Input: bool = false;

fn main() {
    let input = false;
    let inputbad = 3;
    test(inp$0);
}
"# , expect ! [[r#"
                lc input bool [type+name+local]
                ex false  [type]
                ex input  [type]
                ex true  [type]
                lc inputbad i32 [local]
                fn main() fn() []
                fn test(…) fn(bool) []
            "#]] ,) ; } # [test] fn enum_detail_includes_record_fields () { check (r#"
enum Foo { Foo { x: i32, y: i32 } }

fn main() { Foo::Fo$0 }
"# , SymbolKind :: Variant , expect ! [[r#"
                [
                    CompletionItem {
                        label: "Foo {…}",
                        detail_left: None,
                        detail_right: Some(
                            "Foo { x: i32, y: i32 }",
                        ),
                        source_range: 54..56,
                        delete: 54..56,
                        insert: "Foo { x: ${1:()}, y: ${2:()} }$0",
                        kind: SymbolKind(
                            Variant,
                        ),
                        lookup: "Foo{}",
                        detail: "Foo { x: i32, y: i32 }",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: None,
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: Some(
                                CompletionRelevanceFn {
                                    has_params: true,
                                    has_self_param: false,
                                    return_type: DirectConstructor,
                                },
                            ),
                            is_skipping_completion: false,
                        },
                        trigger_call_info: true,
                    },
                ]
            "#]] ,) ; } # [test] fn enum_detail_includes_tuple_fields () { check (r#"
enum Foo { Foo (i32, i32) }

fn main() { Foo::Fo$0 }
"# , SymbolKind :: Variant , expect ! [[r#"
                [
                    CompletionItem {
                        label: "Foo(…)",
                        detail_left: None,
                        detail_right: Some(
                            "Foo(i32, i32)",
                        ),
                        source_range: 46..48,
                        delete: 46..48,
                        insert: "Foo(${1:()}, ${2:()})$0",
                        kind: SymbolKind(
                            Variant,
                        ),
                        lookup: "Foo()",
                        detail: "Foo(i32, i32)",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: None,
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: Some(
                                CompletionRelevanceFn {
                                    has_params: true,
                                    has_self_param: false,
                                    return_type: DirectConstructor,
                                },
                            ),
                            is_skipping_completion: false,
                        },
                        trigger_call_info: true,
                    },
                ]
            "#]] ,) ; } # [test] fn fn_detail_includes_args_and_return_type () { check (r#"
fn foo<T>(a: u32, b: u32, t: T) -> (u32, T) { (a, t) }

fn main() { fo$0 }
"# , SymbolKind :: Function , expect ! [[r#"
                [
                    CompletionItem {
                        label: "foo(…)",
                        detail_left: None,
                        detail_right: Some(
                            "fn(u32, u32, T) -> (u32, T)",
                        ),
                        source_range: 68..70,
                        delete: 68..70,
                        insert: "foo(${1:a}, ${2:b}, ${3:t})$0",
                        kind: SymbolKind(
                            Function,
                        ),
                        lookup: "foo",
                        detail: "fn(u32, u32, T) -> (u32, T)",
                        trigger_call_info: true,
                    },
                    CompletionItem {
                        label: "main()",
                        detail_left: None,
                        detail_right: Some(
                            "fn()",
                        ),
                        source_range: 68..70,
                        delete: 68..70,
                        insert: "main();$0",
                        kind: SymbolKind(
                            Function,
                        ),
                        lookup: "main",
                        detail: "fn()",
                    },
                ]
            "#]] ,) ; } # [test] fn fn_detail_includes_variadics () { check (r#"
unsafe extern "C" fn foo(a: u32, b: u32, ...) {}

fn main() { fo$0 }
"# , SymbolKind :: Function , expect ! [[r#"
                [
                    CompletionItem {
                        label: "foo(…)",
                        detail_left: None,
                        detail_right: Some(
                            "unsafe fn(u32, u32, ...)",
                        ),
                        source_range: 62..64,
                        delete: 62..64,
                        insert: "foo(${1:a}, ${2:b});$0",
                        kind: SymbolKind(
                            Function,
                        ),
                        lookup: "foo",
                        detail: "unsafe fn(u32, u32, ...)",
                        trigger_call_info: true,
                    },
                    CompletionItem {
                        label: "main()",
                        detail_left: None,
                        detail_right: Some(
                            "fn()",
                        ),
                        source_range: 62..64,
                        delete: 62..64,
                        insert: "main();$0",
                        kind: SymbolKind(
                            Function,
                        ),
                        lookup: "main",
                        detail: "fn()",
                    },
                ]
            "#]] ,) ; } # [test] fn enum_detail_just_name_for_unit () { check (r#"
enum Foo { Foo }

fn main() { Foo::Fo$0 }
"# , SymbolKind :: Variant , expect ! [[r#"
                [
                    CompletionItem {
                        label: "Foo",
                        detail_left: None,
                        detail_right: Some(
                            "Foo",
                        ),
                        source_range: 35..37,
                        delete: 35..37,
                        insert: "Foo$0",
                        kind: SymbolKind(
                            Variant,
                        ),
                        detail: "Foo",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: None,
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: Some(
                                CompletionRelevanceFn {
                                    has_params: false,
                                    has_self_param: false,
                                    return_type: DirectConstructor,
                                },
                            ),
                            is_skipping_completion: false,
                        },
                        trigger_call_info: true,
                    },
                ]
            "#]] ,) ; } # [test] fn lookup_enums_by_two_qualifiers () { check_kinds (r#"
mod m {
    pub enum Spam { Foo, Bar(i32) }
}
fn main() { let _: m::Spam = S$0 }
"# , & [CompletionItemKind :: SymbolKind (SymbolKind :: Function) , CompletionItemKind :: SymbolKind (SymbolKind :: Module) , CompletionItemKind :: SymbolKind (SymbolKind :: Variant) ,] , expect ! [[r#"
                [
                    CompletionItem {
                        label: "main()",
                        detail_left: None,
                        detail_right: Some(
                            "fn()",
                        ),
                        source_range: 75..76,
                        delete: 75..76,
                        insert: "main();$0",
                        kind: SymbolKind(
                            Function,
                        ),
                        lookup: "main",
                        detail: "fn()",
                    },
                    CompletionItem {
                        label: "m",
                        detail_left: None,
                        detail_right: None,
                        source_range: 75..76,
                        delete: 75..76,
                        insert: "m",
                        kind: SymbolKind(
                            Module,
                        ),
                    },
                    CompletionItem {
                        label: "m::Spam::Bar(…)",
                        detail_left: None,
                        detail_right: Some(
                            "m::Spam::Bar(i32)",
                        ),
                        source_range: 75..76,
                        delete: 75..76,
                        insert: "m::Spam::Bar(${1:()})$0",
                        kind: SymbolKind(
                            Variant,
                        ),
                        lookup: "Spam::Bar()",
                        detail: "m::Spam::Bar(i32)",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: Some(
                                Exact,
                            ),
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: Some(
                                CompletionRelevanceFn {
                                    has_params: true,
                                    has_self_param: false,
                                    return_type: DirectConstructor,
                                },
                            ),
                            is_skipping_completion: false,
                        },
                        trigger_call_info: true,
                    },
                    CompletionItem {
                        label: "m::Spam::Foo",
                        detail_left: None,
                        detail_right: Some(
                            "m::Spam::Foo",
                        ),
                        source_range: 75..76,
                        delete: 75..76,
                        insert: "m::Spam::Foo$0",
                        kind: SymbolKind(
                            Variant,
                        ),
                        lookup: "Spam::Foo",
                        detail: "m::Spam::Foo",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: Some(
                                Exact,
                            ),
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: Some(
                                CompletionRelevanceFn {
                                    has_params: false,
                                    has_self_param: false,
                                    return_type: DirectConstructor,
                                },
                            ),
                            is_skipping_completion: false,
                        },
                        trigger_call_info: true,
                    },
                ]
            "#]] ,) } # [test] fn sets_deprecated_flag_in_items () { check (r#"
#[deprecated]
fn something_deprecated() {}

fn main() { som$0 }
"# , SymbolKind :: Function , expect ! [[r#"
                [
                    CompletionItem {
                        label: "main()",
                        detail_left: None,
                        detail_right: Some(
                            "fn()",
                        ),
                        source_range: 56..59,
                        delete: 56..59,
                        insert: "main();$0",
                        kind: SymbolKind(
                            Function,
                        ),
                        lookup: "main",
                        detail: "fn()",
                    },
                    CompletionItem {
                        label: "something_deprecated()",
                        detail_left: None,
                        detail_right: Some(
                            "fn()",
                        ),
                        source_range: 56..59,
                        delete: 56..59,
                        insert: "something_deprecated();$0",
                        kind: SymbolKind(
                            Function,
                        ),
                        lookup: "something_deprecated",
                        detail: "fn()",
                        deprecated: true,
                    },
                ]
            "#]] ,) ; check (r#"
struct A { #[deprecated] the_field: u32 }
fn foo() { A { the$0 } }
"# , SymbolKind :: Field , expect ! [[r#"
                [
                    CompletionItem {
                        label: "the_field",
                        detail_left: None,
                        detail_right: Some(
                            "u32",
                        ),
                        source_range: 57..60,
                        delete: 57..60,
                        insert: "the_field",
                        kind: SymbolKind(
                            Field,
                        ),
                        detail: "u32",
                        deprecated: true,
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: Some(
                                CouldUnify,
                            ),
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: None,
                            is_skipping_completion: false,
                        },
                    },
                ]
            "#]] ,) ; } # [test] fn renders_docs () { check_kinds (r#"
struct S {
    /// Field docs
    foo:
}
impl S {
    /// Method docs
    fn bar(self) { self.$0 }
}"# , & [CompletionItemKind :: SymbolKind (SymbolKind :: Method) , CompletionItemKind :: SymbolKind (SymbolKind :: Field) ,] , expect ! [[r#"
                [
                    CompletionItem {
                        label: "bar()",
                        detail_left: None,
                        detail_right: Some(
                            "fn(self)",
                        ),
                        source_range: 94..94,
                        delete: 94..94,
                        insert: "bar();$0",
                        kind: SymbolKind(
                            Method,
                        ),
                        lookup: "bar",
                        detail: "fn(self)",
                        documentation: Documentation(
                            "Method docs",
                        ),
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: None,
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: Some(
                                CompletionRelevanceFn {
                                    has_params: true,
                                    has_self_param: true,
                                    return_type: Other,
                                },
                            ),
                            is_skipping_completion: false,
                        },
                    },
                    CompletionItem {
                        label: "foo",
                        detail_left: None,
                        detail_right: Some(
                            "{unknown}",
                        ),
                        source_range: 94..94,
                        delete: 94..94,
                        insert: "foo",
                        kind: SymbolKind(
                            Field,
                        ),
                        detail: "{unknown}",
                        documentation: Documentation(
                            "Field docs",
                        ),
                    },
                ]
            "#]] ,) ; check_kinds (r#"
use self::my$0;

/// mod docs
mod my { }

/// enum docs
enum E {
    /// variant docs
    V
}
use self::E::*;
"# , & [CompletionItemKind :: SymbolKind (SymbolKind :: Module) , CompletionItemKind :: SymbolKind (SymbolKind :: Variant) , CompletionItemKind :: SymbolKind (SymbolKind :: Enum) ,] , expect ! [[r#"
                [
                    CompletionItem {
                        label: "my",
                        detail_left: None,
                        detail_right: None,
                        source_range: 10..12,
                        delete: 10..12,
                        insert: "my",
                        kind: SymbolKind(
                            Module,
                        ),
                        documentation: Documentation(
                            "mod docs",
                        ),
                    },
                    CompletionItem {
                        label: "V",
                        detail_left: None,
                        detail_right: Some(
                            "V",
                        ),
                        source_range: 10..12,
                        delete: 10..12,
                        insert: "V$0",
                        kind: SymbolKind(
                            Variant,
                        ),
                        detail: "V",
                        documentation: Documentation(
                            "variant docs",
                        ),
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: None,
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: Some(
                                CompletionRelevanceFn {
                                    has_params: false,
                                    has_self_param: false,
                                    return_type: DirectConstructor,
                                },
                            ),
                            is_skipping_completion: false,
                        },
                        trigger_call_info: true,
                    },
                    CompletionItem {
                        label: "E",
                        detail_left: None,
                        detail_right: Some(
                            "E",
                        ),
                        source_range: 10..12,
                        delete: 10..12,
                        insert: "E",
                        kind: SymbolKind(
                            Enum,
                        ),
                        detail: "E",
                        documentation: Documentation(
                            "enum docs",
                        ),
                    },
                ]
            "#]] ,) } # [test] fn dont_render_attrs () { check (r#"
struct S;
impl S {
    #[inline]
    fn the_method(&self) { }
}
fn foo(s: S) { s.$0 }
"# , CompletionItemKind :: SymbolKind (SymbolKind :: Method) , expect ! [[r#"
                [
                    CompletionItem {
                        label: "the_method()",
                        detail_left: None,
                        detail_right: Some(
                            "fn(&self)",
                        ),
                        source_range: 81..81,
                        delete: 81..81,
                        insert: "the_method();$0",
                        kind: SymbolKind(
                            Method,
                        ),
                        lookup: "the_method",
                        detail: "fn(&self)",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: None,
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: Some(
                                CompletionRelevanceFn {
                                    has_params: true,
                                    has_self_param: true,
                                    return_type: Other,
                                },
                            ),
                            is_skipping_completion: false,
                        },
                    },
                ]
            "#]] ,) } # [test] fn no_call_parens_if_fn_ptr_needed () { cov_mark :: check ! (no_call_parens_if_fn_ptr_needed) ; check_edit ("foo" , r#"
fn foo(foo: u8, bar: u8) {}
struct ManualVtable { f: fn(u8, u8) }

fn main() -> ManualVtable {
    ManualVtable { f: f$0 }
}
"# , r#"
fn foo(foo: u8, bar: u8) {}
struct ManualVtable { f: fn(u8, u8) }

fn main() -> ManualVtable {
    ManualVtable { f: foo }
}
"# ,) ; check_edit ("type" , r#"
struct RawIdentTable { r#type: u32 }

fn main() -> RawIdentTable {
    RawIdentTable { t$0: 42 }
}
"# , r#"
struct RawIdentTable { r#type: u32 }

fn main() -> RawIdentTable {
    RawIdentTable { r#type: 42 }
}
"# ,) ; } # [test] fn no_parens_in_use_item () { check_edit ("foo" , r#"
mod m { pub fn foo() {} }
use crate::m::f$0;
"# , r#"
mod m { pub fn foo() {} }
use crate::m::foo;
"# ,) ; } # [test] fn no_parens_in_call () { check_edit ("foo" , r#"
fn foo(x: i32) {}
fn main() { f$0(); }
"# , r#"
fn foo(x: i32) {}
fn main() { foo(); }
"# ,) ; check_edit ("foo" , r#"
struct Foo;
impl Foo { fn foo(&self){} }
fn f(foo: &Foo) { foo.f$0(); }
"# , r#"
struct Foo;
impl Foo { fn foo(&self){} }
fn f(foo: &Foo) { foo.foo(); }
"# ,) ; } # [test] fn inserts_angle_brackets_for_generics () { cov_mark :: check ! (inserts_angle_brackets_for_generics) ; check_edit ("Vec" , r#"
struct Vec<T> {}
fn foo(xs: Ve$0)
"# , r#"
struct Vec<T> {}
fn foo(xs: Vec<$0>)
"# ,) ; check_edit ("Vec" , r#"
type Vec<T> = (T,);
fn foo(xs: Ve$0)
"# , r#"
type Vec<T> = (T,);
fn foo(xs: Vec<$0>)
"# ,) ; check_edit ("Vec" , r#"
struct Vec<T = i128> {}
fn foo(xs: Ve$0)
"# , r#"
struct Vec<T = i128> {}
fn foo(xs: Vec)
"# ,) ; check_edit ("Vec" , r#"
struct Vec<T> {}
fn foo(xs: Ve$0<i128>)
"# , r#"
struct Vec<T> {}
fn foo(xs: Vec<i128>)
"# ,) ; } # [test] fn active_param_relevance () { check_relevance (r#"
struct S { foo: i64, bar: u32, baz: u32 }
fn test(bar: u32) { }
fn foo(s: S) { test(s.$0) }
"# , expect ! [[r#"
                fd bar u32 [type+name]
                fd baz u32 [type]
                fd foo i64 []
            "#]] ,) ; } # [test] fn record_field_relevances () { check_relevance (r#"
struct A { foo: i64, bar: u32, baz: u32 }
struct B { x: (), y: f32, bar: u32 }
fn foo(a: A) { B { bar: a.$0 }; }
"# , expect ! [[r#"
                fd bar u32 [type+name]
                fd baz u32 [type]
                fd foo i64 []
            "#]] ,) } # [test] fn tuple_field_detail () { check (r#"
struct S(i32);

fn f() -> i32 {
    let s = S(0);
    s.0$0
}
"# , SymbolKind :: Field , expect ! [[r#"
                [
                    CompletionItem {
                        label: "0",
                        detail_left: None,
                        detail_right: Some(
                            "i32",
                        ),
                        source_range: 56..57,
                        delete: 56..57,
                        insert: "0",
                        kind: SymbolKind(
                            Field,
                        ),
                        detail: "i32",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: Some(
                                Exact,
                            ),
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: None,
                            is_skipping_completion: false,
                        },
                    },
                ]
            "#]] ,) ; } # [test] fn record_field_and_call_relevances () { check_relevance (r#"
struct A { foo: i64, bar: u32, baz: u32 }
struct B { x: (), y: f32, bar: u32 }
fn f(foo: i64) {  }
fn foo(a: A) { B { bar: f(a.$0) }; }
"# , expect ! [[r#"
                fd foo i64 [type+name]
                fd bar u32 []
                fd baz u32 []
            "#]] ,) ; check_relevance (r#"
struct A { foo: i64, bar: u32, baz: u32 }
struct B { x: (), y: f32, bar: u32 }
fn f(foo: i64) {  }
fn foo(a: A) { f(B { bar: a.$0 }); }
"# , expect ! [[r#"
                fd bar u32 [type+name]
                fd baz u32 [type]
                fd foo i64 []
            "#]] ,) ; } # [test] fn prioritize_exact_ref_match () { check_relevance (r#"
struct WorldSnapshot { _f: () };
fn go(world: &WorldSnapshot) { go(w$0) }
"# , expect ! [[r#"
                lc world &WorldSnapshot [type+name+local]
                ex world  [type]
                st WorldSnapshot {…} WorldSnapshot { _f: () } []
                st &WorldSnapshot {…} [type]
                st WorldSnapshot WorldSnapshot []
                st &WorldSnapshot [type]
                fn go(…) fn(&WorldSnapshot) []
            "#]] ,) ; } # [test] fn too_many_arguments () { cov_mark :: check ! (too_many_arguments) ; check_relevance (r#"
struct Foo;
fn f(foo: &Foo) { f(foo, w$0) }
"# , expect ! [[r#"
                lc foo &Foo [local]
                st Foo Foo []
                fn f(…) fn(&Foo) []
            "#]] ,) ; } # [test] fn score_fn_type_and_name_match () { check_relevance (r#"
struct A { bar: u8 }
fn baz() -> u8 { 0 }
fn bar() -> u8 { 0 }
fn f() { A { bar: b$0 }; }
"# , expect ! [[r#"
                fn bar() fn() -> u8 [type+name]
                ex bar()  [type]
                fn baz() fn() -> u8 [type]
                ex baz()  [type]
                st A A []
                fn f() fn() []
            "#]] ,) ; } # [test] fn score_method_type_and_name_match () { check_relevance (r#"
fn baz(aaa: u32){}
struct Foo;
impl Foo {
fn aaa(&self) -> u32 { 0 }
fn bbb(&self) -> u32 { 0 }
fn ccc(&self) -> u64 { 0 }
}
fn f() {
    baz(Foo.$0
}
"# , expect ! [[r#"
                me aaa() fn(&self) -> u32 [type+name]
                me bbb() fn(&self) -> u32 [type]
                me ccc() fn(&self) -> u64 []
            "#]] ,) ; } # [test] fn score_method_name_match_only () { check_relevance (r#"
fn baz(aaa: u32){}
struct Foo;
impl Foo {
fn aaa(&self) -> u64 { 0 }
}
fn f() {
    baz(Foo.$0
}
"# , expect ! [[r#"
                me aaa() fn(&self) -> u64 [name]
            "#]] ,) ; } # [test] fn test_avoid_redundant_suggestion () { check_relevance (r#"
struct aa([u8]);

impl aa {
    fn from_bytes(bytes: &[u8]) -> &Self {
        unsafe { &*(bytes as *const [u8] as *const aa) }
    }
}

fn bb()-> &'static aa {
    let bytes = b"hello";
    aa::$0
}
"# , expect ! [[r#"
                ex bb()  [type]
                fn from_bytes(…) fn(&[u8]) -> &aa [type_could_unify]
            "#]] ,) ; } # [test] fn suggest_ref_mut () { cov_mark :: check ! (suggest_ref) ; check_relevance (r#"
struct S;
fn foo(s: &mut S) {}
fn main() {
    let mut s = S;
    foo($0);
}
            "# , expect ! [[r#"
                lc s S [name+local]
                lc &mut s [type+name+local]
                st S S []
                st &mut S [type]
                st S S []
                st &mut S [type]
                fn foo(…) fn(&mut S) []
                fn main() fn() []
            "#]] ,) ; check_relevance (r#"
struct S;
fn foo(s: &mut S) {}
fn main() {
    let mut s = S;
    foo(&mut $0);
}
            "# , expect ! [[r#"
                lc s S [type+name+local]
                st S S [type]
                st S S [type]
                ex S  [type]
                ex s  [type]
                fn foo(…) fn(&mut S) []
                fn main() fn() []
            "#]] ,) ; check_relevance (r#"
struct S;
fn foo(s: &mut S) {}
fn main() {
    let mut ssss = S;
    foo(&mut s$0);
}
            "# , expect ! [[r#"
                st S S [type]
                lc ssss S [type+local]
                st S S [type]
                ex S  [type]
                ex ssss  [type]
                fn foo(…) fn(&mut S) []
                fn main() fn() []
            "#]] ,) ; } # [test] fn suggest_deref_copy () { cov_mark :: check ! (suggest_deref) ; check_relevance (r#"
//- minicore: copy
struct Foo;

impl Copy for Foo {}
impl Clone for Foo {
    fn clone(&self) -> Self { *self }
}

fn bar(x: Foo) {}

fn main() {
    let foo = &Foo;
    bar($0);
}
"# , expect ! [[r#"
                st Foo Foo [type]
                st Foo Foo [type]
                ex Foo  [type]
                lc foo &Foo [local]
                lc *foo [type+local]
                tt Clone  []
                tt Copy  []
                fn bar(…) fn(Foo) []
                md core  []
                fn main() fn() []
            "#]] ,) ; } # [test] fn suggest_deref_trait () { check_relevance (r#"
//- minicore: deref
struct S;
struct T(S);

impl core::ops::Deref for T {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn foo(s: &S) {}

fn main() {
    let t = T(S);
    let m = 123;

    foo($0);
}
            "# , expect ! [[r#"
                st S S []
                st &S [type]
                ex core::ops::Deref::deref(&t)  [type_could_unify]
                lc m i32 [local]
                lc t T [local]
                lc &t [type+local]
                st S S []
                st &S [type]
                st T T []
                st &T [type]
                md core  []
                fn foo(…) fn(&S) []
                fn main() fn() []
            "#]] ,) } # [test] fn suggest_deref_mut () { check_relevance (r#"
//- minicore: deref_mut
struct S;
struct T(S);

impl core::ops::Deref for T {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for T {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn foo(s: &mut S) {}

fn main() {
    let t = T(S);
    let m = 123;

    foo($0);
}
            "# , expect ! [[r#"
                st S S []
                st &mut S [type]
                ex core::ops::DerefMut::deref_mut(&mut t)  [type_could_unify]
                lc m i32 [local]
                lc t T [local]
                lc &mut t [type+local]
                st S S []
                st &mut S [type]
                st T T []
                st &mut T [type]
                md core  []
                fn foo(…) fn(&mut S) []
                fn main() fn() []
            "#]] ,) } # [test] fn locals () { check_relevance (r#"
fn foo(bar: u32) {
    let baz = 0;

    f$0
}
"# , expect ! [[r#"
                lc bar u32 [local]
                lc baz i32 [local]
                fn foo(…) fn(u32) []
            "#]] ,) ; } # [test] fn enum_owned () { check_relevance (r#"
enum Foo { A, B }
fn foo() {
    bar($0);
}
fn bar(t: Foo) {}
"# , expect ! [[r#"
                ev Foo::A Foo::A [type]
                ev Foo::B Foo::B [type]
                en Foo Foo [type]
                ex Foo::A  [type]
                ex Foo::B  [type]
                fn bar(…) fn(Foo) []
                fn foo() fn() []
            "#]] ,) ; } # [test] fn enum_ref () { check_relevance (r#"
enum Foo { A, B }
fn foo() {
    bar($0);
}
fn bar(t: &Foo) {}
"# , expect ! [[r#"
                ev Foo::A Foo::A []
                ev &Foo::A [type]
                ev Foo::B Foo::B []
                ev &Foo::B [type]
                en Foo Foo []
                en &Foo [type]
                fn bar(…) fn(&Foo) []
                fn foo() fn() []
            "#]] ,) ; } # [test] fn suggest_deref_fn_ret () { check_relevance (r#"
//- minicore: deref
struct S;
struct T(S);

impl core::ops::Deref for T {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn foo(s: &S) {}
fn bar() -> T {}

fn main() {
    foo($0);
}
"# , expect ! [[r#"
                st S S []
                st &S [type]
                ex core::ops::Deref::deref(&bar())  [type_could_unify]
                st S S []
                st &S [type]
                st T T []
                st &T [type]
                fn bar() fn() -> T []
                fn &bar() [type]
                md core  []
                fn foo(…) fn(&S) []
                fn main() fn() []
            "#]] ,) } # [test] fn op_function_relevances () { check_relevance (r#"
#[lang = "sub"]
trait Sub {
    fn sub(self, other: Self) -> Self { self }
}
impl Sub for u32 {}
fn foo(a: u32) { a.$0 }
"# , expect ! [[r#"
                me sub(…) fn(self, Self) -> Self [op_method]
            "#]] ,) ; check_relevance (r#"
struct Foo;
impl Foo {
    fn new() -> Self {}
}
#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool;
}

impl PartialEq for Foo {}
fn main() {
    Foo::$0
}
"# , expect ! [[r#"
                fn new() fn() -> Foo []
                me eq(…) fn(&self, &Rhs) -> bool [op_method]
                me ne(…) fn(&self, &Rhs) -> bool [op_method]
            "#]] ,) ; } # [test] fn constructor_order_simple () { check_relevance (r#"
struct Foo;
struct Other;
struct Option<T>(T);

impl Foo {
    fn fn_ctr() -> Foo { unimplemented!() }
    fn fn_another(n: u32) -> Other { unimplemented!() }
    fn fn_ctr_self() -> Option<Self> { unimplemented!() }
}

fn test() {
    let a = Foo::$0;
}
"# , expect ! [[r#"
                fn fn_ctr() fn() -> Foo [type_could_unify]
                fn fn_ctr_self() fn() -> Option<Foo> [type_could_unify]
                fn fn_another(…) fn(u32) -> Other [type_could_unify]
            "#]] ,) ; } # [test] fn constructor_order_kind () { check_function_relevance (r#"
struct Foo;
struct Bar;
struct Option<T>(T);
enum Result<T, E> { Ok(T), Err(E) };

impl Foo {
    fn fn_ctr(&self) -> Foo { unimplemented!() }
    fn fn_ctr_with_args(&self, n: u32) -> Foo { unimplemented!() }
    fn fn_another(&self, n: u32) -> Bar { unimplemented!() }
    fn fn_ctr_wrapped(&self, ) -> Option<Self> { unimplemented!() }
    fn fn_ctr_wrapped_2(&self, ) -> Result<Self, Bar> { unimplemented!() }
    fn fn_ctr_wrapped_3(&self, ) -> Result<Bar, Self> { unimplemented!() } // Self is not the first type
    fn fn_ctr_wrapped_with_args(&self, m: u32) -> Option<Self> { unimplemented!() }
    fn fn_another_unit(&self) { unimplemented!() }
}

fn test() {
    let a = self::Foo::$0;
}
"# , expect ! [[r#"
                [
                    (
                        "fn(&self, u32) -> Bar",
                        Some(
                            CompletionRelevanceFn {
                                has_params: true,
                                has_self_param: true,
                                return_type: Other,
                            },
                        ),
                    ),
                    (
                        "fn(&self)",
                        Some(
                            CompletionRelevanceFn {
                                has_params: true,
                                has_self_param: true,
                                return_type: Other,
                            },
                        ),
                    ),
                    (
                        "fn(&self) -> Foo",
                        Some(
                            CompletionRelevanceFn {
                                has_params: true,
                                has_self_param: true,
                                return_type: DirectConstructor,
                            },
                        ),
                    ),
                    (
                        "fn(&self, u32) -> Foo",
                        Some(
                            CompletionRelevanceFn {
                                has_params: true,
                                has_self_param: true,
                                return_type: DirectConstructor,
                            },
                        ),
                    ),
                    (
                        "fn(&self) -> Option<Foo>",
                        Some(
                            CompletionRelevanceFn {
                                has_params: true,
                                has_self_param: true,
                                return_type: Constructor,
                            },
                        ),
                    ),
                    (
                        "fn(&self) -> Result<Foo, Bar>",
                        Some(
                            CompletionRelevanceFn {
                                has_params: true,
                                has_self_param: true,
                                return_type: Constructor,
                            },
                        ),
                    ),
                    (
                        "fn(&self) -> Result<Bar, Foo>",
                        Some(
                            CompletionRelevanceFn {
                                has_params: true,
                                has_self_param: true,
                                return_type: Constructor,
                            },
                        ),
                    ),
                    (
                        "fn(&self, u32) -> Option<Foo>",
                        Some(
                            CompletionRelevanceFn {
                                has_params: true,
                                has_self_param: true,
                                return_type: Constructor,
                            },
                        ),
                    ),
                ]
            "#]] ,) ; } # [test] fn constructor_order_relevance () { check_relevance (r#"
struct Foo;
struct FooBuilder;
struct Result<T>(T);

impl Foo {
    fn fn_no_ret(&self) {}
    fn fn_ctr_with_args(input: u32) -> Foo { unimplemented!() }
    fn fn_direct_ctr() -> Self { unimplemented!() }
    fn fn_ctr() -> Result<Self> { unimplemented!() }
    fn fn_other() -> Result<u32> { unimplemented!() }
    fn fn_builder() -> FooBuilder { unimplemented!() }
}

fn test() {
    let a = self::Foo::$0;
}
"# , expect ! [[r#"
                fn fn_direct_ctr() fn() -> Foo [type_could_unify]
                fn fn_ctr_with_args(…) fn(u32) -> Foo [type_could_unify]
                fn fn_builder() fn() -> FooBuilder [type_could_unify]
                fn fn_ctr() fn() -> Result<Foo> [type_could_unify]
                me fn_no_ret(…) fn(&self) [type_could_unify]
                fn fn_other() fn() -> Result<u32> [type_could_unify]
            "#]] ,) ; } # [test] fn function_relevance_generic_1 () { check_relevance (r#"
struct Foo<T: Default>(T);
struct FooBuilder;
struct Option<T>(T);
enum Result<T, E>{Ok(T), Err(E)};

impl<T: Default> Foo<T> {
    fn fn_returns_unit(&self) {}
    fn fn_ctr_with_args(input: T) -> Foo<T> { unimplemented!() }
    fn fn_direct_ctr() -> Self { unimplemented!() }
    fn fn_ctr_wrapped() -> Option<Self> { unimplemented!() }
    fn fn_ctr_wrapped_2() -> Result<Self, u32> { unimplemented!() }
    fn fn_other() -> Option<u32> { unimplemented!() }
    fn fn_builder() -> FooBuilder { unimplemented!() }
}

fn test() {
    let a = self::Foo::<u32>::$0;
}
                "# , expect ! [[r#"
                fn fn_direct_ctr() fn() -> Foo<T> [type_could_unify]
                fn fn_ctr_with_args(…) fn(T) -> Foo<T> [type_could_unify]
                fn fn_builder() fn() -> FooBuilder [type_could_unify]
                fn fn_ctr_wrapped() fn() -> Option<Foo<T>> [type_could_unify]
                fn fn_ctr_wrapped_2() fn() -> Result<Foo<T>, u32> [type_could_unify]
                fn fn_other() fn() -> Option<u32> [type_could_unify]
                me fn_returns_unit(…) fn(&self) [type_could_unify]
            "#]] ,) ; } # [test] fn function_relevance_generic_2 () { check_relevance (r#"
struct Foo<T: Default>(T);
struct FooBuilder;
struct Option<T>(T);
enum Result<T, E>{Ok(T), Err(E)};

impl<T: Default> Foo<T> {
    fn fn_no_ret(&self) {}
    fn fn_ctr_with_args(input: T) -> Foo<T> { unimplemented!() }
    fn fn_direct_ctr() -> Self { unimplemented!() }
    fn fn_ctr() -> Option<Self> { unimplemented!() }
    fn fn_ctr2() -> Result<Self, u32> { unimplemented!() }
    fn fn_other() -> Option<u32> { unimplemented!() }
    fn fn_builder() -> FooBuilder { unimplemented!() }
}

fn test() {
    let a : Res<Foo<u32>> = Foo::$0;
}
                "# , expect ! [[r#"
                fn fn_direct_ctr() fn() -> Foo<T> [type_could_unify]
                fn fn_ctr_with_args(…) fn(T) -> Foo<T> [type_could_unify]
                fn fn_builder() fn() -> FooBuilder [type_could_unify]
                fn fn_ctr() fn() -> Option<Foo<T>> [type_could_unify]
                fn fn_ctr2() fn() -> Result<Foo<T>, u32> [type_could_unify]
                me fn_no_ret(…) fn(&self) [type_could_unify]
                fn fn_other() fn() -> Option<u32> [type_could_unify]
            "#]] ,) ; } # [test] fn struct_field_method_ref () { check_kinds (r#"
struct Foo { bar: u32, qux: fn() }
impl Foo { fn baz(&self) -> u32 { 0 } }

fn foo(f: Foo) { let _: &u32 = f.b$0 }
"# , & [CompletionItemKind :: SymbolKind (SymbolKind :: Method) , CompletionItemKind :: SymbolKind (SymbolKind :: Field) ,] , expect ! [[r#"
                [
                    CompletionItem {
                        label: "baz()",
                        detail_left: None,
                        detail_right: Some(
                            "fn(&self) -> u32",
                        ),
                        source_range: 109..110,
                        delete: 109..110,
                        insert: "baz()$0",
                        kind: SymbolKind(
                            Method,
                        ),
                        lookup: "baz",
                        detail: "fn(&self) -> u32",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: None,
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: Some(
                                CompletionRelevanceFn {
                                    has_params: true,
                                    has_self_param: true,
                                    return_type: Other,
                                },
                            ),
                            is_skipping_completion: false,
                        },
                        ref_match: "&@107",
                    },
                    CompletionItem {
                        label: "bar",
                        detail_left: None,
                        detail_right: Some(
                            "u32",
                        ),
                        source_range: 109..110,
                        delete: 109..110,
                        insert: "bar",
                        kind: SymbolKind(
                            Field,
                        ),
                        detail: "u32",
                        ref_match: "&@107",
                    },
                    CompletionItem {
                        label: "qux",
                        detail_left: None,
                        detail_right: Some(
                            "fn()",
                        ),
                        source_range: 109..110,
                        text_edit: TextEdit {
                            indels: [
                                Indel {
                                    insert: "(",
                                    delete: 107..107,
                                },
                                Indel {
                                    insert: "qux)()",
                                    delete: 109..110,
                                },
                            ],
                            annotation: None,
                        },
                        kind: SymbolKind(
                            Field,
                        ),
                        detail: "fn()",
                    },
                ]
            "#]] ,) ; } # [test] fn expected_fn_type_ref () { check_kinds (r#"
struct S { field: fn() }

fn foo() {
    let foo: fn() = S { fields: || {}}.fi$0;
}
"# , & [CompletionItemKind :: SymbolKind (SymbolKind :: Field)] , expect ! [[r#"
                [
                    CompletionItem {
                        label: "field",
                        detail_left: None,
                        detail_right: Some(
                            "fn()",
                        ),
                        source_range: 76..78,
                        delete: 76..78,
                        insert: "field",
                        kind: SymbolKind(
                            Field,
                        ),
                        detail: "fn()",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: Some(
                                Exact,
                            ),
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: None,
                            is_skipping_completion: false,
                        },
                    },
                ]
            "#]] ,) } # [test] fn qualified_path_ref () { check_kinds (r#"
struct S;

struct T;
impl T {
    fn foo() -> S {}
}

fn bar(s: &S) {}

fn main() {
    bar(T::$0);
}
"# , & [CompletionItemKind :: SymbolKind (SymbolKind :: Function)] , expect ! [[r#"
                [
                    CompletionItem {
                        label: "foo()",
                        detail_left: None,
                        detail_right: Some(
                            "fn() -> S",
                        ),
                        source_range: 95..95,
                        delete: 95..95,
                        insert: "foo()$0",
                        kind: SymbolKind(
                            Function,
                        ),
                        lookup: "foo",
                        detail: "fn() -> S",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: None,
                            is_local: false,
                            trait_: None,
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: Some(
                                CompletionRelevanceFn {
                                    has_params: false,
                                    has_self_param: false,
                                    return_type: Other,
                                },
                            ),
                            is_skipping_completion: false,
                        },
                        ref_match: "&@92",
                    },
                ]
            "#]] ,) ; } # [test] fn generic_enum () { check_relevance (r#"
enum Foo<T> { A(T), B }
// bar() should not be an exact type match
// because the generic parameters are different
fn bar() -> Foo<u8> { Foo::B }
// FIXME baz() should be an exact type match
// because the types could unify, but it currently
// is not. This is due to the T here being
// TyKind::Placeholder rather than TyKind::Missing.
fn baz<T>() -> Foo<T> { Foo::B }
fn foo() {
    let foo: Foo<u32> = Foo::B;
    let _: Foo<u32> = f$0;
}
"# , expect ! [[r#"
                ev Foo::B Foo::B [type_could_unify]
                ev Foo::A(…) Foo::A(T) [type_could_unify]
                lc foo Foo<u32> [type+local]
                ex Foo::B  [type]
                ex foo  [type]
                en Foo Foo<{unknown}> [type_could_unify]
                fn bar() fn() -> Foo<u8> []
                fn baz() fn() -> Foo<T> []
                fn foo() fn() []
            "#]] ,) ; } # [test] fn postfix_exact_match_is_high_priority () { cov_mark :: check ! (postfix_exact_match_is_high_priority) ; check_relevance_for_kinds (r#"
mod ops {
    pub trait Not {
        type Output;
        fn not(self) -> Self::Output;
    }

    impl Not for bool {
        type Output = bool;
        fn not(self) -> bool { if self { false } else { true }}
    }
}

fn main() {
    let _: bool = (9 > 2).not$0;
}
    "# , & [CompletionItemKind :: Snippet , CompletionItemKind :: SymbolKind (SymbolKind :: Method)] , expect ! [[r#"
                sn not !expr [snippet]
                sn box Box::new(expr) []
                sn call function(expr) []
                sn const const {} []
                sn dbg dbg!(expr) []
                sn dbgr dbg!(&expr) []
                sn deref *expr []
                sn if if expr {} []
                sn match match expr {} []
                sn ref &expr []
                sn refm &mut expr []
                sn return return expr []
                sn unsafe unsafe {} []
                sn while while expr {} []
                me not() fn(self) -> <Self as Not>::Output [requires_import]
            "#]] ,) ; } # [test] fn postfix_inexact_match_is_low_priority () { cov_mark :: check ! (postfix_inexact_match_is_low_priority) ; check_relevance_for_kinds (r#"
struct S;
impl S {
    fn f(&self) {}
}
fn main() {
    S.$0
}
    "# , & [CompletionItemKind :: Snippet , CompletionItemKind :: SymbolKind (SymbolKind :: Method)] , expect ! [[r#"
                me f() fn(&self) []
                sn box Box::new(expr) []
                sn call function(expr) []
                sn const const {} []
                sn dbg dbg!(expr) []
                sn dbgr dbg!(&expr) []
                sn deref *expr []
                sn let let []
                sn letm let mut []
                sn match match expr {} []
                sn ref &expr []
                sn refm &mut expr []
                sn return return expr []
                sn unsafe unsafe {} []
            "#]] ,) ; } # [test] fn flyimport_reduced_relevance () { check_relevance (r#"
mod std {
    pub mod io {
        pub trait BufRead {}
        pub struct BufReader;
        pub struct BufWriter;
    }
}
struct Buffer;

fn f() {
    Buf$0
}
"# , expect ! [[r#"
                st Buffer Buffer []
                fn f() fn() []
                md std  []
                tt BufRead  [requires_import]
                st BufReader BufReader [requires_import]
                st BufWriter BufWriter [requires_import]
            "#]] ,) ; } # [test] fn completes_struct_with_raw_identifier () { check_edit ("type" , r#"
mod m { pub struct r#type {} }
fn main() {
    let r#type = m::t$0;
}
"# , r#"
mod m { pub struct r#type {} }
fn main() {
    let r#type = m::r#type;
}
"# ,) } # [test] fn completes_fn_with_raw_identifier () { check_edit ("type" , r#"
mod m { pub fn r#type {} }
fn main() {
    m::t$0
}
"# , r#"
mod m { pub fn r#type {} }
fn main() {
    m::r#type();$0
}
"# ,) } # [test] fn completes_macro_with_raw_identifier () { check_edit ("let!" , r#"
macro_rules! r#let { () => {} }
fn main() {
    $0
}
"# , r#"
macro_rules! r#let { () => {} }
fn main() {
    r#let!($0)
}
"# ,) } # [test] fn completes_variant_with_raw_identifier () { check_edit ("type" , r#"
enum A { r#type }
fn main() {
    let a = A::t$0
}
"# , r#"
enum A { r#type }
fn main() {
    let a = A::r#type$0
}
"# ,) } # [test] fn completes_field_with_raw_identifier () { check_edit ("fn" , r#"
mod r#type {
    pub struct r#struct {
        pub r#fn: u32
    }
}

fn main() {
    let a = r#type::r#struct {};
    a.$0
}
"# , r#"
mod r#type {
    pub struct r#struct {
        pub r#fn: u32
    }
}

fn main() {
    let a = r#type::r#struct {};
    a.r#fn
}
"# ,) } # [test] fn completes_const_with_raw_identifier () { check_edit ("type" , r#"
struct r#struct {}
impl r#struct { pub const r#type: u8 = 1; }
fn main() {
    r#struct::t$0
}
"# , r#"
struct r#struct {}
impl r#struct { pub const r#type: u8 = 1; }
fn main() {
    r#struct::r#type
}
"# ,) } # [test] fn completes_type_alias_with_raw_identifier () { check_edit ("type type" , r#"
struct r#struct {}
trait r#trait { type r#type; }
impl r#trait for r#struct { type t$0 }
"# , r#"
struct r#struct {}
trait r#trait { type r#type; }
impl r#trait for r#struct { type r#type = $0; }
"# ,) } # [test] fn field_access_includes_self () { check_edit ("length" , r#"
struct S {
    length: i32
}

impl S {
    fn some_fn(&self) {
        let l = len$0
    }
}
"# , r#"
struct S {
    length: i32
}

impl S {
    fn some_fn(&self) {
        let l = self.length
    }
}
"# ,) } # [test] fn notable_traits_method_relevance () { check_kinds (r#"
#[doc(notable_trait)]
trait Write {
    fn write(&self);
    fn flush(&self);
}

struct Writer;

impl Write for Writer {
    fn write(&self) {}
    fn flush(&self) {}
}

fn main() {
    Writer.$0
}
"# , & [CompletionItemKind :: SymbolKind (SymbolKind :: Method) , CompletionItemKind :: SymbolKind (SymbolKind :: Field) , CompletionItemKind :: SymbolKind (SymbolKind :: Function) ,] , expect ! [[r#"
                [
                    CompletionItem {
                        label: "flush()",
                        detail_left: Some(
                            "(as Write)",
                        ),
                        detail_right: Some(
                            "fn(&self)",
                        ),
                        source_range: 193..193,
                        delete: 193..193,
                        insert: "flush();$0",
                        kind: SymbolKind(
                            Method,
                        ),
                        lookup: "flush",
                        detail: "fn(&self)",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: None,
                            is_local: false,
                            trait_: Some(
                                CompletionRelevanceTraitInfo {
                                    notable_trait: true,
                                    is_op_method: false,
                                },
                            ),
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: None,
                            is_skipping_completion: false,
                        },
                    },
                    CompletionItem {
                        label: "write()",
                        detail_left: Some(
                            "(as Write)",
                        ),
                        detail_right: Some(
                            "fn(&self)",
                        ),
                        source_range: 193..193,
                        delete: 193..193,
                        insert: "write();$0",
                        kind: SymbolKind(
                            Method,
                        ),
                        lookup: "write",
                        detail: "fn(&self)",
                        relevance: CompletionRelevance {
                            exact_name_match: false,
                            type_match: None,
                            is_local: false,
                            trait_: Some(
                                CompletionRelevanceTraitInfo {
                                    notable_trait: true,
                                    is_op_method: false,
                                },
                            ),
                            is_name_already_imported: false,
                            requires_import: false,
                            is_private_editable: false,
                            postfix_match: None,
                            function: None,
                            is_skipping_completion: false,
                        },
                    },
                ]
            "#]] ,) ; } }
};
}
