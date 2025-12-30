// Generated macro for Serializer (struct)
macro_rules! Depcrate_seSerializer {
() => {
// Module: crate::se
// Provides: {"Serializer"}
// Dependencies: {}
# [doc = " A Serializer."] # [doc = ""] # [doc = " Returns the classification of the last written type."] pub struct Serializer < 'w , 'r , W : Write > { ser : ContentSerializer < 'w , 'r , W > , # [doc = " Name of the root tag. If not specified, deduced from the structure name"] root_tag : Option < XmlName < 'r > > , }
};
}
