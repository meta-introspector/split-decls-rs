// Generated macro for tests (module)
macro_rules! Depcrate_builder_fieldtests {
() => {
// Module: crate::builder_field
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [allow (unused_imports)] use super :: * ; # [test] fn setter_enabled () { let field = default_builder_field ! () ; assert_eq ! (quote ! (# field) . to_string () , quote ! (# [some_attr] pub foo : :: db :: export :: core :: option :: Option < String >,) . to_string ()) ; } # [test] fn setter_disabled () { let mut field = default_builder_field ! () ; field . field_visibility = Cow :: Owned (syn :: Visibility :: Inherited) ; field . field_type = match field . field_type { BuilderFieldType :: Optional (ty) => BuilderFieldType :: Phantom (ty) , _ => panic ! () , } ; assert_eq ! (quote ! (# field) . to_string () , quote ! (# [some_attr] foo : :: db :: export :: core :: marker :: PhantomData < String >,) . to_string ()) ; } # [test] fn private_field () { let private = Cow :: Owned (syn :: Visibility :: Inherited) ; let mut field = default_builder_field ! () ; field . field_visibility = private ; assert_eq ! (quote ! (# field) . to_string () , quote ! (# [some_attr] foo : :: db :: export :: core :: option :: Option < String >,) . to_string ()) ; } }
};
}
