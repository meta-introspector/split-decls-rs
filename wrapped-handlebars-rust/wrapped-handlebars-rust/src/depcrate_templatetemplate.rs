// Generated macro for Template (struct)
macro_rules! Depcrate_templateTemplate {
() => {
// Module: crate::template
// Provides: {"Template"}
// Dependencies: {}
# [doc = " A handlebars template"] # [non_exhaustive] # [derive (Builder , PartialEq , Eq , Clone , Debug , Default)] pub struct Template { # [builder (setter (into , strip_option) , default)] pub name : Option < String > , pub elements : Vec < TemplateElement > , pub mapping : Vec < TemplateMapping > , }
};
}
