// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl Fixture { fn new (index : usize , ty : Box < syn :: Type >) -> syn :: Result < Self > { let identifier = syn :: Ident :: new (& format ! ("__googletest__fixture__{index}") , ty . span ()) ; match & * ty { Type :: Reference (reference) => Ok (Self { identifier , ty : reference . elem . clone () , kind : if reference . mutability . is_some () { FixtureKind :: MutableRef } else { FixtureKind :: SharedRef } , }) , Type :: Path (..) => Ok (Self { identifier , ty , kind : FixtureKind :: Consumable }) , _ => Err (syn :: Error :: new (ty . span () , "Unexpected fixture type. Only references (&T or &mut T) and paths (T) are supported." ,)) , } } fn wrap_call (& self , inner_call : proc_macro2 :: TokenStream) -> proc_macro2 :: TokenStream { let Self { identifier , ty , kind } = self ; let ref_method = match kind { FixtureKind :: Consumable => { return quote ! { # [allow (non_snake_case)] let # identifier = <# ty as googletest :: fixtures :: ConsumableFixture >:: set_up () ?; { # inner_call } } ; } FixtureKind :: MutableRef => quote ! { . as_mut () } , FixtureKind :: SharedRef => quote ! { . as_ref () } , } ; quote ! { { # [allow (non_snake_case , unused_mut)] let mut # identifier = googletest :: __internal_macro_support :: FixtureTearDownOnDrop :: new (<# ty as googletest :: fixtures :: Fixture >:: set_up () ?) ; let result = { # [allow (non_snake_case)] let # identifier = # identifier # ref_method ; # inner_call } ; # identifier . tear_down () ?; result } } } }
};
}
