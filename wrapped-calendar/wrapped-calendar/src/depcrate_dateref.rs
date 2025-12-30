// Generated macro for Ref (struct)
macro_rules! Depcrate_dateRef {
() => {
// Module: crate::date
// Provides: {"Ref"}
// Dependencies: {}
# [doc = " This exists as a wrapper around `&'a T` so that"] # [doc = " `Date<&'a C>` is possible for calendar `C`."] # [doc = ""] # [doc = " Unfortunately,"] # [doc = " [`AsCalendar`] cannot be implemented on `&'a T` directly because"] # [doc = " `&'a T` is `#[fundamental]` and the impl would clash with the one above with"] # [doc = " `AsCalendar` for `C: Calendar`."] # [doc = ""] # [doc = " Use `Date<Ref<'a, C>>` where you would use `Date<&'a C>`"] # [allow (clippy :: exhaustive_structs)] # [derive (PartialEq , Eq , Debug)] pub struct Ref < 'a , C > (pub & 'a C) ;
};
}
