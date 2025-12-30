// Generated macro for impl_27 (impl)
macro_rules! Depcrate_commonimpl_27 {
() => {
// Module: crate::common
// Provides: {"impl_27"}
// Dependencies: {}
impl BinaryOp { pub fn from_str (s : & str) -> Option < Self > { Some (match s { "Add" => Self :: Add , "BitAnd" => Self :: BitAnd , "BitOr" => Self :: BitOr , "BitXor" => Self :: BitXor , "Div" => Self :: Div , "Mul" => Self :: Mul , "Rem" => Self :: Rem , "Shl" => Self :: Shl , "Shr" => Self :: Shr , "Sub" => Self :: Sub , _ => return None , }) } pub fn to_str (self) -> & 'static str { match self { Self :: Add => "Add" , Self :: BitAnd => "BitAnd" , Self :: BitOr => "BitOr" , Self :: BitXor => "BitXor" , Self :: Div => "Div" , Self :: Mul => "Mul" , Self :: Rem => "Rem" , Self :: Shl => "Shl" , Self :: Shr => "Shr" , Self :: Sub => "Sub" , } } pub fn to_func_name (self) -> & 'static str { match self { Self :: Add => "add" , Self :: BitAnd => "bitand" , Self :: BitOr => "bitor" , Self :: BitXor => "bitxor" , Self :: Div => "div" , Self :: Mul => "mul" , Self :: Rem => "rem" , Self :: Shl => "shl" , Self :: Shr => "shr" , Self :: Sub => "sub" , } } }
};
}
