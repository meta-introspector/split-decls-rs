// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl PhfHash for ParsedKey { fn phf_hash < H > (& self , state : & mut H) where H : Hasher , { match self { ParsedKey :: Str (s) => s . phf_hash (state) , ParsedKey :: Binary (s) => s . phf_hash (state) , ParsedKey :: Char (s) => s . phf_hash (state) , ParsedKey :: I8 (s) => s . phf_hash (state) , ParsedKey :: I16 (s) => s . phf_hash (state) , ParsedKey :: I32 (s) => s . phf_hash (state) , ParsedKey :: I64 (s) => s . phf_hash (state) , ParsedKey :: I128 (s) => s . phf_hash (state) , ParsedKey :: Isize (s) => s . phf_hash (state) , ParsedKey :: U8 (s) => s . phf_hash (state) , ParsedKey :: U16 (s) => s . phf_hash (state) , ParsedKey :: U32 (s) => s . phf_hash (state) , ParsedKey :: U64 (s) => s . phf_hash (state) , ParsedKey :: U128 (s) => s . phf_hash (state) , ParsedKey :: Usize (s) => s . phf_hash (state) , ParsedKey :: Bool (s) => s . phf_hash (state) , ParsedKey :: Tuple (elements) => { for element in elements { element . phf_hash (state) ; } } # [cfg (feature = "unicase")] ParsedKey :: UniCase (s) => s . phf_hash (state) , # [cfg (feature = "unicase")] ParsedKey :: UniCaseAscii (s) => s . phf_hash (state) , # [cfg (feature = "uncased")] ParsedKey :: Uncased (s) => s . phf_hash (state) , } } }
};
}
