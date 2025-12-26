// In lib.rs, we need to declare and re-export the helper macros.
// Each of these will be in their own file.
#[macro_export]
#[doc(hidden)]
#[macro_use] // Required for macro_rules! to be available within other macros
mod mut_visit_with_one_arm;

#[macro_export]
#[doc(hidden)]
#[macro_use]
mod mut_visit_with_many_arm;

#[macro_export]
#[doc(hidden)]
#[macro_use]
mod add_placeholders_arm;

#[macro_export]
#[doc(hidden)]
#[macro_use]
mod visit_with_one_arm;

#[macro_export]
#[doc(hidden)]
#[macro_use]
mod visit_with_many_arm;

#[macro_export]
#[doc(hidden)]
#[macro_use]
mod to_string_one_arm;

#[macro_export]
#[doc(hidden)]
#[macro_use]
mod to_string_many_arm;


// This is the main macro, also re-exported.
// It uses the helper macros internally.
#[macro_export]
#[doc(hidden)]
#[macro_use]
mod ast_fragments;
