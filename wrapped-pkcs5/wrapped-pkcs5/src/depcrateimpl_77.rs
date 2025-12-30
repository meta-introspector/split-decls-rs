// Generated macro for impl_77 (impl)
macro_rules! Depcrateimpl_77 {
() => {
// Module: crate
// Provides: {"impl_77"}
// Dependencies: {}
impl EncodeValue for EncryptionScheme { fn value_len (& self) -> der :: Result < Length > { match self { Self :: Pbes1 (pbes1) => pbes1 . oid () . encoded_len () ? + pbes1 . parameters . encoded_len () ? , Self :: Pbes2 (pbes2) => pbes2 :: PBES2_OID . encoded_len () ? + pbes2 . encoded_len () ? , } } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { match self { Self :: Pbes1 (pbes1) => { pbes1 . oid () . encode (writer) ? ; pbes1 . parameters . encode (writer) ? ; } Self :: Pbes2 (pbes2) => { pbes2 :: PBES2_OID . encode (writer) ? ; pbes2 . encode (writer) ? ; } } Ok (()) } }
};
}
