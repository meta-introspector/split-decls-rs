// Generated macro for impl_99 (impl)
macro_rules! Depcrate_internal_description_rendererimpl_99 {
() => {
// Module: crate::internal::description_renderer
// Provides: {"impl_99"}
// Dependencies: {}
impl Block { fn nested (inner : List) -> Self { Self :: Nested (inner) } fn render (& self , f : & mut dyn Write , indentation : usize , prefix : Cow < 'static , str >) -> Result { match self { Self :: Literal (fragments) => { if fragments . is_empty () { return Ok (()) ; } write ! (f , "{:indentation$}{prefix}" , "") ? ; fragments [0] . render (f) ? ; let block_indentation = indentation + prefix . as_ref () . len () ; for fragment in & fragments [1 ..] { writeln ! (f) ? ; write ! (f , "{:block_indentation$}" , "") ? ; fragment . render (f) ? ; } Ok (()) } Self :: Nested (inner) => inner . render_with_prefix (f , indentation + INDENTATION_SIZE . saturating_sub (prefix . len ()) , prefix ,) , } } }
};
}
