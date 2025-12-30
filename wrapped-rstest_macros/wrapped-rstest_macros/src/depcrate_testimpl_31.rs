// Generated macro for impl_31 (impl)
macro_rules! Depcrate_testimpl_31 {
() => {
// Module: crate::test
// Provides: {"impl_31"}
// Dependencies: {}
impl PatBuilder for syn :: Pat { fn with_mut (self) -> Self { match self { Pat :: Ident (mut ident) => { ident . mutability = Some ("mut" . ast ()) ; syn :: Pat :: Ident (ident) } _ => unimplemented ! ("Unsupported pattern: {:?}" , self ,) , } } }
};
}
