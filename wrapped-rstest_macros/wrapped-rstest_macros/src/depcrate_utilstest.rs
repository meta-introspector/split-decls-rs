// Generated macro for test (module)
macro_rules! Depcrate_utilstest {
() => {
// Module: crate::utils
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use syn :: parse_quote ; use super :: * ; use crate :: test :: { assert_eq , * } ; # [test] fn fn_args_has_pat_should () { let item_fn = parse_quote ! { fn the_function (first : u32 , second : u32) { } } ; assert ! (fn_args_has_pat (& item_fn , & pat ("first"))) ; assert ! (! fn_args_has_pat (& item_fn , & pat ("third"))) ; } # [rstest] # [case :: base ("fn foo<A, B, C>(a: A) -> B {}" , & ["A" , "B"])] # [case :: use_const_in_array ("fn foo<A, const B: usize, C>(a: A) -> [u32; B] {}" , & ["A" , "B" , "u32"])] # [case :: in_type_args ("fn foo<A, const B: usize, C>(a: A) -> SomeType<B> {}" , & ["A" , "B"])] # [case :: in_type_args ("fn foo<A, const B: usize, C>(a: SomeType<A>, b: SomeType<B>) {}" , & ["A" , "B"])] # [case :: pointers ("fn foo<A, B, C>(a: *const A, b: &B) {}" , & ["A" , "B"])] # [case :: lifetime ("fn foo<'a, A, B, C>(a: A, b: &'a B) {}" , & ["a" , "A" , "B"])] # [case :: transitive_lifetime ("fn foo<'a, A, B, C>(a: A, b: B) where B: Iterator<Item=A> + 'a {}" , & ["a" , "A" , "B"])] # [case :: associated ("fn foo<'a, A:Copy, C>(b: impl Iterator<Item=A> + 'a) {}" , & ["a" , "A"])] # [case :: transitive_in_defs ("fn foo<A:Copy, B: Iterator<Item=A>>(b: B) {}" , & ["A" , "B"])] # [case :: transitive_in_where ("fn foo<A:Copy, B>(b: B) where B: Iterator<Item=A> {}" , & ["A" , "B"])] # [case :: transitive_const ("fn foo<const A: usize, B, C>(b: B) where B: Some<A> {}" , & ["A" , "B"])] # [case :: transitive_lifetime ("fn foo<'a, A, B, C>(a: A, b: B) where B: Iterator<Item=A> + 'a {}" , & ["a" , "A" , "B"])] # [case :: transitive_lifetime (r#"fn foo<'a, 'b, 'c, 'd, A, B, C>
        (a: A, b: B) 
        where B: Iterator<Item=A> + 'c, 
        'c: 'a + 'b {}"# , & ["a" , "b" , "c" , "A" , "B"])] fn references_ident_types_should (# [case] f : & str , # [case] expected : & [& str]) { let f : ItemFn = f . ast () ; let used = references_ident_types (& f . sig . generics , f . sig . inputs . iter () , & f . sig . output) ; let expected = to_idents ! (expected) . into_iter () . collect :: < std :: collections :: HashSet < _ > > () ; assert_eq ! (expected , used) ; } # [rstest] # [case :: remove_not_in_output (r#"fn test<R: AsRef<str>, B, F, H: Iterator<Item=u32>>() -> (H, B, String, &str)
                        where F: ToString,
                        B: Borrow<u32>
                        {}"# , r#"fn test<B, H: Iterator<Item=u32>>() -> (H, B, String, &str)
                        where B: Borrow<u32>
                {}"#)] # [case :: not_remove_used_in_arguments (r#"fn test<R: AsRef<str>, B, F, H: Iterator<Item=u32>>
                    (h: H, it: impl Iterator<Item=R>, j: &[B])
                    where F: ToString,
                    B: Borrow<u32>
                {}"# , r#"fn test<R: AsRef<str>, B, H: Iterator<Item=u32>>
                    (h: H, it: impl Iterator<Item=R>, j: &[B])
                    where
                    B: Borrow<u32>
                {}"#)] # [case :: dont_remove_transitive (r#"fn test<A, B, C, D, const F: usize, O>(a: A) where 
            B: AsRef<C>,
            A: Iterator<Item=[B; F]>,
            D: ArsRef<O> {}"# , r#"fn test<A, B, C, const F: usize>(a: A) where 
            B: AsRef<C>,
            A: Iterator<Item=[B; F]> {}"#)] # [case :: remove_unused_lifetime ("fn test<'a, 'b, 'c, 'd, 'e, 'f, 'g, A>(a: &'a uint32, b: impl AsRef<A> + 'b) where 'b: 'c + 'd, A: Copy + 'e, 'f: 'g {}" , "fn test<'a, 'b, 'c, 'd, 'e, A>(a: &'a uint32, b: impl AsRef<A> + 'b) where 'b: 'c + 'd, A: Copy + 'e {}")] # [case :: remove_unused_const (r#"fn test<const A: usize, const B: usize, const C: usize, const D: usize, T, O>
            (a: [u32; A], b: SomeType<B>, c: T) where 
            T: Iterator<Item=[i32; C]>,
            O: AsRef<D> 
            {}"# , r#"fn test<const A: usize, const B: usize, const C: usize, T>
            (a: [u32; A], b: SomeType<B>, c: T) where 
            T: Iterator<Item=[i32; C]>
            {}"#)] fn generics_cleaner (# [case] code : & str , # [case] expected : & str) { let item_fn : ItemFn = code . ast () ; let expected : ItemFn = expected . ast () ; let cleaned = generics_clean_up (& item_fn . sig . generics , item_fn . sig . inputs . iter () , & item_fn . sig . output ,) ; assert_eq ! (expected . sig . generics , cleaned) ; } # [rstest] # [case ("1" , "1")] # [case (r#""1""# , "__1__")] # [case (r#"Some::SomeElse"# , "Some__SomeElse")] # [case (r#""minnie".to_owned()"# , "__minnie___to_owned__")] # [case (r#"vec![1 ,   2, 
    3]"# , "vec__1_2_3_")] # [case (r#"some_macro!("first", {second}, [third])"# , "some_macro____first____second___third__")] # [case (r#"'x'"# , "__x__")] # [case :: ops (r#"a*b+c/d-e%f^g"# , "a_b_c_d_e_f_g")] fn sanitaze_ident_name (# [case] expression : impl AsRef < str > , # [case] expected : impl AsRef < str >) { assert_eq ! (expected . as_ref () , sanitize_ident (expression . as_ref ())) ; } }
};
}
