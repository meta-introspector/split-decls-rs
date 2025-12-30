// Generated macro for impl_229 (impl)
macro_rules! Depcrate_codepointtrie_serdeimpl_229 {
() => {
// Module: crate::codepointtrie::serde
// Provides: {"impl_229"}
// Dependencies: {}
impl < T : TrieValue + Serialize > Serialize for CodePointTrie < '_ , T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let ser = CodePointTrieSerde { header : self . header , index : ZeroFrom :: zero_from (& self . index) , data : ZeroFrom :: zero_from (& self . data) , } ; ser . serialize (serializer) } }
};
}
