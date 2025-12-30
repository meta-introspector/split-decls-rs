// Generated macro for impl_92 (impl)
macro_rules! Depcrate_item_typeimpl_92 {
() => {
// Module: crate::item_type
// Provides: {"impl_92"}
// Dependencies: {}
impl CompareOp { const VARIANTS : & 'static [Self] = & [Self :: Ord , Self :: PartialOrd , Self :: Eq , Self :: PartialEq , Self :: Hash ,] ; fn from_str (s : & str) -> Option < Self > { Some (match s { "Ord" => Self :: Ord , "PartialOrd" => Self :: PartialOrd , "Eq" => Self :: Eq , "PartialEq" => Self :: PartialEq , "Hash" => Self :: Hash , _ => return None , }) } fn to_path (self) -> Path { match self { CompareOp :: Ord => parse_quote ! (:: core :: cmp :: Ord) , CompareOp :: PartialOrd => parse_quote ! (:: core :: cmp :: PartialOrd) , CompareOp :: Eq => parse_quote ! (:: core :: cmp :: Eq) , CompareOp :: PartialEq => parse_quote ! (:: core :: cmp :: PartialEq) , CompareOp :: Hash => parse_quote ! (:: core :: hash :: Hash) , } } fn to_str (self) -> & 'static str { match self { CompareOp :: Ord => "Ord" , CompareOp :: PartialOrd => "PartialOrd" , CompareOp :: Eq => "Eq" , CompareOp :: PartialEq => "PartialEq" , CompareOp :: Hash => "Hash" , } } fn to_str_snake_case (self) -> & 'static str { match self { CompareOp :: Ord => "ord" , CompareOp :: PartialOrd => "partial_ord" , CompareOp :: Eq => "eq" , CompareOp :: PartialEq => "partial_eq" , CompareOp :: Hash => "hash" , } } fn is_effects_to (self , target : Self) -> bool { matches ! ((target , self) , (Self :: Ord , Self :: Ord) | (Self :: PartialOrd , Self :: PartialOrd | Self :: Ord) | (Self :: Eq , Self :: Eq | Self :: Ord) | (Self :: PartialEq , Self :: PartialEq | Self :: Eq | Self :: PartialOrd | Self :: Ord) | (Self :: Hash , Self :: Hash | Self :: Eq | Self :: Ord)) } }
};
}
