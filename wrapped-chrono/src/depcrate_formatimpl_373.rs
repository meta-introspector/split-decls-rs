// Generated macro for impl_373 (impl)
macro_rules! Depcrate_formatimpl_373 {
() => {
// Module: crate::format
// Provides: {"impl_373"}
// Dependencies: {}
# [cfg (feature = "defmt")] impl < 'a > defmt :: Format for Item < 'a > { fn format (& self , f : defmt :: Formatter) { match self { Item :: Literal (v) => defmt :: write ! (f , "Literal {{ {} }}" , v) , # [cfg (feature = "alloc")] Item :: OwnedLiteral (_) => { } Item :: Space (v) => defmt :: write ! (f , "Space {{ {}  }}" , v) , # [cfg (feature = "alloc")] Item :: OwnedSpace (_) => { } Item :: Numeric (u , v) => defmt :: write ! (f , "Numeric {{ {}, {} }}" , u , v) , Item :: Fixed (v) => defmt :: write ! (f , "Fixed {{ {}  }}" , v) , Item :: Error => defmt :: write ! (f , "Error") , } } }
};
}
