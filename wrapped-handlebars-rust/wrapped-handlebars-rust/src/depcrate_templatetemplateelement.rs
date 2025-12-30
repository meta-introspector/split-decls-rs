// Generated macro for TemplateElement (enum)
macro_rules! Depcrate_templateTemplateElement {
() => {
// Module: crate::template
// Provides: {"TemplateElement"}
// Dependencies: {}
# [non_exhaustive] # [derive (PartialEq , Eq , Clone , Debug)] pub enum TemplateElement { RawString (String) , HtmlExpression (Box < HelperTemplate >) , Expression (Box < HelperTemplate >) , HelperBlock (Box < HelperTemplate >) , DecoratorExpression (Box < DecoratorTemplate >) , DecoratorBlock (Box < DecoratorTemplate >) , PartialExpression (Box < DecoratorTemplate >) , PartialBlock (Box < DecoratorTemplate >) , Comment (String) , }
};
}
