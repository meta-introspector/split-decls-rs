// Generated macro for impl_89 (impl)
macro_rules! Depcrate_item_typeimpl_89 {
() => {
// Module: crate::item_type
// Provides: {"impl_89"}
// Dependencies: {}
impl UnaryOp { fn from_str (s : & str) -> Option < Self > { Some (match s { "Neg" => Self :: Neg , "Not" => Self :: Not , _ => return None , }) } fn to_str (self) -> & 'static str { match self { UnaryOp :: Neg => "Neg" , UnaryOp :: Not => "Not" , } } fn to_func_name (self) -> & 'static str { match self { UnaryOp :: Neg => "neg" , UnaryOp :: Not => "not" , } } }
};
}
