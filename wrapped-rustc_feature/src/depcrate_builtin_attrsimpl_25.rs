// Generated macro for impl_25 (impl)
macro_rules! Depcrate_builtin_attrsimpl_25 {
() => {
// Module: crate::builtin_attrs
// Provides: {"impl_25"}
// Dependencies: {}
impl AttributeTemplate { pub fn suggestions (& self , style : AttrStyle , name : impl std :: fmt :: Display) -> Vec < String > { let mut suggestions = vec ! [] ; let inner = match style { AttrStyle :: Outer => "" , AttrStyle :: Inner => "!" , } ; if self . word { suggestions . push (format ! ("#{inner}[{name}]")) ; } if let Some (descr) = self . list { for descr in descr { suggestions . push (format ! ("#{inner}[{name}({descr})]")) ; } } suggestions . extend (self . one_of . iter () . map (| & word | format ! ("#{inner}[{name}({word})]"))) ; if let Some (descr) = self . name_value_str { for descr in descr { suggestions . push (format ! ("#{inner}[{name} = \"{descr}\"]")) ; } } suggestions . sort () ; suggestions } }
};
}
