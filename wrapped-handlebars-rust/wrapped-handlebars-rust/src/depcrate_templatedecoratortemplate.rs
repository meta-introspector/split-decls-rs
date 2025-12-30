// Generated macro for DecoratorTemplate (struct)
macro_rules! Depcrate_templateDecoratorTemplate {
() => {
// Module: crate::template
// Provides: {"DecoratorTemplate"}
// Dependencies: {}
# [non_exhaustive] # [derive (Builder , PartialEq , Eq , Clone , Debug)] pub struct DecoratorTemplate { pub name : Parameter , pub params : Vec < Parameter > , pub hash : HashMap < String , Parameter > , # [builder (setter (strip_option) , default)] pub template : Option < Template > , # [builder (setter (into , strip_option) , default)] pub indent : Option < String > , pub (crate) indent_before_write : bool , }
};
}
