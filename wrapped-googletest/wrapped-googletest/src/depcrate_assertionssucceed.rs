// Generated macro for succeed (macro)
macro_rules! Depcrate_assertionssucceed {
() => {
// Module: crate::assertions
// Provides: {"succeed"}
// Dependencies: {}
# [doc = " Generates a success. This **does not** make the overall test succeed. A test"] # [doc = " is only considered successful if none of its assertions fail during its"] # [doc = " execution."] # [doc = ""] # [doc = " The succeed!() assertion is purely documentary. The only user visible output"] # [doc = " is a stdout with information on where the success was generated from."] # [doc = ""] # [doc = " ```ignore"] # [doc = " fn test_to_be_implemented() {"] # [doc = "     succeed!();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " One may include formatted arguments in the success message:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " fn test_to_be_implemented() {"] # [doc = "     succeed!(\"I am just a fake test: {}\", \"a fake test indeed\");"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! succeed { ($ ($ message : expr) ,+ $ (,) ?) => { { println ! ("{}\n at {}:{}:{}" , format ! ($ ($ message) ,*) , file ! () , line ! () , column ! ()) ; } } ; () => { $ crate :: succeed ! ("Success") } ; }
};
}
