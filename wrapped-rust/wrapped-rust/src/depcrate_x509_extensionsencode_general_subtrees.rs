// Generated macro for encode_general_subtrees (function)
macro_rules! Depcrate_x509_extensionsencode_general_subtrees {
() => {
// Module: crate::x509::extensions
// Provides: {"encode_general_subtrees"}
// Dependencies: {}
fn encode_general_subtrees < 'a > (py : pyo3 :: Python < '_ > , ka_bytes : & 'a cryptography_keepalive :: KeepAlive < pyo3 :: pybacked :: PyBackedBytes > , ka_str : & 'a cryptography_keepalive :: KeepAlive < pyo3 :: pybacked :: PyBackedStr > , subtrees : & pyo3 :: Bound < 'a , pyo3 :: PyAny > ,) -> Result < Option < extensions :: SequenceOfSubtrees < 'a , Asn1Write > > , CryptographyError > { if subtrees . is_none () { Ok (None) } else { let mut subtree_seq = vec ! [] ; for name in subtrees . try_iter () ? { let gn = x509 :: common :: encode_general_name (py , ka_bytes , ka_str , & name ?) ? ; subtree_seq . push (extensions :: GeneralSubtree { base : gn , minimum : 0 , maximum : None , }) ; } Ok (Some (asn1 :: SequenceOfWriter :: new (subtree_seq))) } }
};
}
