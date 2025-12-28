macro_rules! match_reordered_struct_instantiation {
    () => {
        # [test] fn match_reordered_struct_instantiation () { assert_matches ("Foo {aa: 1, b: 2, ccc: 3}" , "struct Foo {} fn f() {Foo {b: 2, ccc: 3, aa: 1}}" , & ["Foo {b: 2, ccc: 3, aa: 1}"] ,) ; assert_no_match ("Foo {a: 1}" , "struct Foo {} fn f() {Foo {b: 1}}") ; assert_no_match ("Foo {a: 1}" , "struct Foo {} fn f() {Foo {a: 2}}") ; assert_no_match ("Foo {a: 1, b: 2}" , "struct Foo {} fn f() {Foo {a: 1}}") ; assert_no_match ("Foo {a: 1, b: 2}" , "struct Foo {} fn f() {Foo {b: 2}}") ; assert_no_match ("Foo {a: 1, }" , "struct Foo {} fn f() {Foo {a: 1, b: 2}}") ; assert_no_match ("Foo {a: 1, z: 9}" , "struct Foo {} fn f() {Foo {a: 1}}") ; }
    };
}

match_reordered_struct_instantiation!()