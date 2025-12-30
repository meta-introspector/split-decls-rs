// Generated macro for TimeTypes (struct)
macro_rules! Depcrate_output_tableTimeTypes {
() => {
// Module: crate::output::table
// Provides: {"TimeTypes"}
// Dependencies: {}
# [doc = " Fields for which of a file’s time fields should be displayed in the"] # [doc = " columns output."] # [doc = ""] # [doc = " There should always be at least one of these — there’s no way to disable"] # [doc = " the time columns entirely (yet)."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] # [rustfmt :: skip] # [allow (clippy :: struct_excessive_bools)] pub struct TimeTypes { pub modified : bool , pub changed : bool , pub accessed : bool , pub created : bool , }
};
}
