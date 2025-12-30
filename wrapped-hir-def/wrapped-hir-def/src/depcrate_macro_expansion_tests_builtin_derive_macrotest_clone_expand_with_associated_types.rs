// Generated macro for test_clone_expand_with_associated_types (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrotest_clone_expand_with_associated_types {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"test_clone_expand_with_associated_types"}
// Dependencies: {}
# [test] fn test_clone_expand_with_associated_types () { check (r#"
//- minicore: derive, clone
trait Trait {
    type InWc;
    type InFieldQualified;
    type InFieldShorthand;
    type InGenericArg;
}
trait Marker {}
struct Vec<T>(T);

#[derive(Clone)]
struct Foo<T: Trait>
where
    <T as Trait>::InWc: Marker,
{
    qualified: <T as Trait>::InFieldQualified,
    shorthand: T::InFieldShorthand,
    generic: Vec<T::InGenericArg>,
}
"# , expect ! [[r#"
trait Trait {
    type InWc;
    type InFieldQualified;
    type InFieldShorthand;
    type InGenericArg;
}
trait Marker {}
struct Vec<T>(T);

#[derive(Clone)]
struct Foo<T: Trait>
where
    <T as Trait>::InWc: Marker,
{
    qualified: <T as Trait>::InFieldQualified,
    shorthand: T::InFieldShorthand,
    generic: Vec<T::InGenericArg>,
}

impl <T: $crate::clone::Clone, > $crate::clone::Clone for Foo<T, > where <T as Trait>::InWc: Marker, T: Trait, T::InFieldShorthand: $crate::clone::Clone, T::InGenericArg: $crate::clone::Clone, {
    fn clone(&self ) -> Self {
        match self {
            Foo {
                qualified: qualified, shorthand: shorthand, generic: generic,
            }
            =>Foo {
                qualified: qualified.clone(), shorthand: shorthand.clone(), generic: generic.clone(),
            }
            ,
        }
    }
}"#]] ,) ; }
};
}
