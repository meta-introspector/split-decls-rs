// Generated macro for Struct (struct)
macro_rules! Depcrate_se_elementStruct {
() => {
// Module: crate::se::element
// Provides: {"Struct"}
// Dependencies: {}
# [doc = " A serializer for struct variants, which serializes the struct contents inside"] # [doc = " of wrapping tags (`<${tag}>...</${tag}>`)."] # [doc = ""] # [doc = " Returns the classification of the last written type."] # [doc = ""] # [doc = " Serialization of each field depends on it representation:"] # [doc = " - attributes written directly to the higher serializer"] # [doc = " - elements buffered into internal buffer and at the end written into higher"] # [doc = "   serializer"] pub struct Struct < 'w , 'k , W : Write > { ser : ElementSerializer < 'w , 'k , W > , # [doc = " Buffer to store serialized elements"] children : String , # [doc = " Whether need to write indent after the last written field"] write_indent : bool , }
};
}
