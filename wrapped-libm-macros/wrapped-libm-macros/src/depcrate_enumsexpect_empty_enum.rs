// Generated macro for expect_empty_enum (function)
macro_rules! Depcrate_enumsexpect_empty_enum {
() => {
// Module: crate::enums
// Provides: {"expect_empty_enum"}
// Dependencies: {}
# [doc = " Verify that an enum is empty, otherwise return an error"] fn expect_empty_enum (item : & ItemEnum) -> syn :: Result < () > { if ! item . variants . is_empty () { Err (syn :: Error :: new (item . variants . span () , "expected an empty enum" ,)) } else { Ok (()) } }
};
}
