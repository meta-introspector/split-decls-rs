// Generated macro for Builder (struct)
macro_rules! Depcrate_mock_functionBuilder {
() => {
// Module: crate::mock_function
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " Build a MockFunction."] # [derive (Clone , Copy , Debug)] pub (crate) struct Builder < 'a > { attrs : & 'a [Attribute] , call_levels : Option < usize > , concretize : bool , levels : usize , parent : Option < & 'a Ident > , sig : & 'a Signature , struct_ : Option < & 'a Ident > , struct_generics : Option < & 'a Generics > , trait_ : Option < & 'a Ident > , vis : & 'a Visibility }
};
}
