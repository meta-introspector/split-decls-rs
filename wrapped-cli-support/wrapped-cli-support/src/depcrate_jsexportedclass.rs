// Generated macro for ExportedClass (struct)
macro_rules! Depcrate_jsExportedClass {
() => {
// Module: crate::js
// Provides: {"ExportedClass"}
// Dependencies: {}
# [derive (Default)] struct ExportedClass { comments : String , contents : String , # [doc = " The TypeScript for the class's methods."] typescript : String , # [doc = " Whether TypeScript for this class should be emitted (i.e., `skip_typescript` wasn't specified)."] generate_typescript : bool , has_constructor : bool , wrap_needed : bool , unwrap_needed : bool , # [doc = " Whether to generate helper methods for inspecting the class"] is_inspectable : bool , # [doc = " All readable properties of the class"] readable_properties : Vec < String > , # [doc = " Map from field to information about those fields"] typescript_fields : HashMap < FieldLocation , FieldInfo > , # [doc = " The namespace to export the class through, if any"] js_namespace : Option < Vec < String > > , }
};
}
