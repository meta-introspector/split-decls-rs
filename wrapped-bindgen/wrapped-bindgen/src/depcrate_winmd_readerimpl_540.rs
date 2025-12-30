// Generated macro for impl_540 (impl)
macro_rules! Depcrate_winmd_readerimpl_540 {
() => {
// Module: crate::winmd::reader
// Provides: {"impl_540"}
// Dependencies: {}
impl Category { fn new (def : TypeDef) -> Self { if let Some (extends) = def . extends () { if extends . namespace () == "System" { match extends . name () { "Enum" => Self :: Enum , "MulticastDelegate" => Self :: Delegate , "ValueType" => Self :: Struct , "Attribute" => Self :: Attribute , _ => Self :: Class , } } else { Self :: Class } } else { Self :: Interface } } }
};
}
