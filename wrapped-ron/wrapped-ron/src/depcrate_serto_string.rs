// Generated macro for to_string (function)
macro_rules! Depcrate_serto_string {
() => {
// Module: crate::ser
// Provides: {"to_string"}
// Dependencies: {}
# [doc = " Serializes `value` and returns it as string."] # [doc = ""] # [doc = " This function does not generate any newlines or nice formatting;"] # [doc = " if you want that, you can use [`to_string_pretty`] instead."] pub fn to_string < T > (value : & T) -> Result < String > where T : ? Sized + Serialize , { Options :: default () . to_string (value) }
};
}
