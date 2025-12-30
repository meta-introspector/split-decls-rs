// Generated macro for Field (struct)
macro_rules! Depcrate_types_executableField {
() => {
// Module: crate::types::executable
// Provides: {"Field"}
// Dependencies: {}
# [doc = " A field being selected on an object, such as `name` or `weightKilos:"] # [doc = " weight(unit: KILOGRAMS)`."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#Field)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct Field { # [doc = " The optional field alias."] pub alias : Option < Positioned < Name > > , # [doc = " The name of the field."] pub name : Positioned < Name > , # [doc = " The arguments to the field, empty if no arguments are provided."] pub arguments : Vec < (Positioned < Name > , Positioned < Value >) > , # [doc = " The directives in the field selector."] pub directives : Vec < Positioned < Directive > > , # [doc = " The subfields being selected in this field, if it is an object. Empty if"] # [doc = " no fields are being selected."] pub selection_set : Positioned < SelectionSet > , }
};
}
