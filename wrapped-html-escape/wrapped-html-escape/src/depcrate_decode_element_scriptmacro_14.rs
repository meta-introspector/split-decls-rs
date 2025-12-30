// Generated macro for macro_14 (macro)
macro_rules! Depcrate_decode_element_scriptmacro_14 {
() => {
// Module: crate::decode::element::script
// Provides: {"macro_14"}
// Dependencies: {}
decode_impl ! { 7 ; # [doc = " The following substring is unescaped:"] # [doc = ""] # [doc = " * `<\\/script>` => `</script>`"] parse_script ; # [doc = " Decode text from the `<script>` element."] decode_script ; # [doc = " Write text from the `<script>` element to a mutable `String` reference and return the encoded string slice."] decode_script_to_string ; # [doc = " Write text from the `<script>` element to a mutable `Vec<u8>` reference and return the encoded data slice."] decode_script_to_vec ; # [doc = " Write text from the `<script>` element to a writer."] decode_script_to_writer ; }
};
}
