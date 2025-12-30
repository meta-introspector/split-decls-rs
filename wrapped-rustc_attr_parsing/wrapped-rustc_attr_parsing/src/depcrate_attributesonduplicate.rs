// Generated macro for OnDuplicate (enum)
macro_rules! Depcrate_attributesOnDuplicate {
() => {
// Module: crate::attributes
// Provides: {"OnDuplicate"}
// Dependencies: {}
pub (crate) enum OnDuplicate < S : Stage > { # [doc = " Give a default warning"] Warn , # [doc = " Duplicates will be a warning, with a note that this will be an error in the future."] WarnButFutureError , # [doc = " Give a default error"] Error , # [doc = " Ignore duplicates"] Ignore , # [doc = " Custom function called when a duplicate attribute is found."] # [doc = ""] # [doc = " - `unused` is the span of the attribute that was unused or bad because of some"] # [doc = "   duplicate reason (see [`AttributeOrder`])"] # [doc = " - `used` is the span of the attribute that was used in favor of the unused attribute"] Custom (fn (cx : & AcceptContext < '_ , '_ , S > , used : Span , unused : Span)) , }
};
}
