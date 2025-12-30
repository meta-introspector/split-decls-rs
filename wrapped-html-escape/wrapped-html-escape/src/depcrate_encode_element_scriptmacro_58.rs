// Generated macro for macro_58 (macro)
macro_rules! Depcrate_encode_element_scriptmacro_58 {
() => {
// Module: crate::encode::element::script
// Provides: {"macro_58"}
// Dependencies: {}
encode_impl ! { 7 ; # [doc = " The following substring is escaped:"] # [doc = ""] # [doc = " * `</script>` => `<\\/script>`"] # [doc = " * `<!--` => `<\\!--`"] parse_script_comment ; # [doc = " Encode text used in the `<script>` element."] encode_script ; # [doc = " Write text used in the `<script>` element to a mutable `String` reference and return the encoded string slice."] encode_script_to_string ; # [doc = " Write text used in the `<script>` element to a mutable `Vec<u8>` reference and return the encoded data slice."] encode_script_to_vec ; # [doc = " Write text used in the `<script>` element to a writer."] encode_script_to_writer ; }
};
}
