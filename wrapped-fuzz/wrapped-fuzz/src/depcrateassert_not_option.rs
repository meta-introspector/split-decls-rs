// Generated macro for assert_not_option (function)
macro_rules! Depcrateassert_not_option {
() => {
// Module: crate
// Provides: {"assert_not_option"}
// Dependencies: {}
# [doc = " Asserts when the provided [`CompactString`] is `std::mem::transmute`-ed to"] # [doc = " `Option<CompactString>` that it is never `None`, and when we unwrap the `Option<CompactString>`,"] # [doc = " it equals the original `CompactString`."] # [doc = ""] # [doc = " We use a bit within the discriminant to store whether or not we're \"None\". We want to make sure"] # [doc = " valid `CompactString`s never set this bit, and thus get misinterpreted as `None`"] fn assert_not_option (compact : CompactString) -> CompactString { let clone = compact . clone () ; let maybe_compact : Option < CompactString > = unsafe { std :: mem :: transmute (clone) } ; assert ! (maybe_compact . is_some ()) ; assert_eq ! (compact , maybe_compact . unwrap ()) ; compact }
};
}
