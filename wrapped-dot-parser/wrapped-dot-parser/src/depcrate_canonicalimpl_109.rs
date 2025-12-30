// Generated macro for impl_109 (impl)
macro_rules! Depcrate_canonicalimpl_109 {
() => {
// Module: crate::canonical
// Provides: {"impl_109"}
// Dependencies: {}
# [cfg (feature = "display")] impl < A > Display for Graph < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { if self . strict { write ! (f , "strict ") ? ; } if self . is_digraph { write ! (f , "digraph ") ? ; } else { write ! (f , "graph ") ? ; } if let Some (name) = & self . name { write ! (f , "{}" , name) ? ; } writeln ! (f , "{{") ? ; for stmt in & self . attr { writeln ! (f , "{}" , stmt) ? ; } for ideq in & self . ideqs { writeln ! (f , "{}" , ideq) ? ; } write ! (f , "{}" , self . nodes) ? ; write ! (f , "{}" , self . edges) ? ; writeln ! (f , "}}") ? ; Result :: Ok (()) } }
};
}
