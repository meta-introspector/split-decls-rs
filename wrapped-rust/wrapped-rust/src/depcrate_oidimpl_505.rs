// Generated macro for impl_505 (impl)
macro_rules! Depcrate_oidimpl_505 {
() => {
// Module: crate::oid
// Provides: {"impl_505"}
// Dependencies: {}
# [pyo3 :: pymethods] impl ObjectIdentifier { # [new] fn new (value : & str) -> CryptographyResult < Self > { let oid = asn1 :: ObjectIdentifier :: from_string (value) . ok_or_else (| | asn1 :: ParseError :: new (asn1 :: ParseErrorKind :: InvalidValue)) ? ; Ok (ObjectIdentifier { oid }) } # [getter] fn dotted_string (& self) -> String { self . oid . to_string () } # [getter] fn _name < 'p > (slf : pyo3 :: PyRef < 'p , Self > , py : pyo3 :: Python < 'p > ,) -> pyo3 :: PyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { types :: OID_NAMES . get (py) ? . call_method1 (pyo3 :: intern ! (py , "get") , (slf , "Unknown OID")) } fn __deepcopy__ (slf : pyo3 :: PyRef < '_ , Self > , _memo : pyo3 :: Py < pyo3 :: PyAny > ,) -> pyo3 :: PyRef < '_ , Self > { slf } fn __repr__ (slf : & pyo3 :: Bound < '_ , Self > , py : pyo3 :: Python < '_ >) -> pyo3 :: PyResult < String > { let name = Self :: _name (slf . borrow () , py) ? ; Ok (format ! ("<ObjectIdentifier(oid={}, name={})>" , slf . get () . oid , name . extract ::< pyo3 :: pybacked :: PyBackedStr > () ?)) } fn __eq__ (& self , other : pyo3 :: PyRef < '_ , ObjectIdentifier >) -> bool { self . oid == other . oid } fn __hash__ (& self) -> u64 { let mut hasher = DefaultHasher :: new () ; self . oid . hash (& mut hasher) ; hasher . finish () } }
};
}
