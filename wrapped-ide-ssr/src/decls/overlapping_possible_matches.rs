macro_rules! overlapping_possible_matches {
    () => {
        # [test] fn overlapping_possible_matches () { assert_matches ("foo(foo($a))" , "fn foo() {} fn main() {foo(foo(foo(foo(42))))}" , & ["foo(foo(42))" , "foo(foo(foo(foo(42))))"] ,) ; }
    };
}

overlapping_possible_matches!();