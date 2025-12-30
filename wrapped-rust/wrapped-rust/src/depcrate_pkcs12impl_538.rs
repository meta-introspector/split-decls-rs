// Generated macro for impl_538 (impl)
macro_rules! Depcrate_pkcs12impl_538 {
() => {
// Module: crate::pkcs12
// Provides: {"impl_538"}
// Dependencies: {}
# [pyo3 :: pymethods] impl PKCS12Certificate { # [new] # [pyo3 (signature = (cert , friendly_name = None))] fn new (cert : pyo3 :: Py < Certificate > , friendly_name : Option < pyo3 :: Py < pyo3 :: types :: PyBytes > > ,) -> PKCS12Certificate { PKCS12Certificate { certificate : cert , friendly_name , } } fn __eq__ (& self , py : pyo3 :: Python < '_ > , other : pyo3 :: PyRef < '_ , Self > ,) -> CryptographyResult < bool > { let friendly_name_eq = match (& self . friendly_name , & other . friendly_name) { (Some (a) , Some (b)) => a . bind (py) . as_bytes () == b . bind (py) . as_bytes () , (None , None) => true , _ => false , } ; Ok (friendly_name_eq && self . certificate . bind (py) . eq (other . certificate . bind (py)) ?) } fn __hash__ (& self , py : pyo3 :: Python < '_ >) -> CryptographyResult < u64 > { let mut hasher = DefaultHasher :: new () ; self . certificate . bind (py) . hash () ? . hash (& mut hasher) ; match & self . friendly_name { Some (v) => v . bind (py) . hash () ? . hash (& mut hasher) , None => None :: < u32 > . hash (& mut hasher) , } ; Ok (hasher . finish ()) } fn __repr__ (& self , py : pyo3 :: Python < '_ >) -> pyo3 :: PyResult < String > { let py_friendly_name_repr ; let friendly_name_repr = match & self . friendly_name { Some (v) => { py_friendly_name_repr = v . bind (py) . repr () ? . extract :: < pyo3 :: pybacked :: PyBackedStr > () ? ; & * py_friendly_name_repr } None => "None" , } ; Ok (format ! ("<PKCS12Certificate({}, friendly_name={})>" , self . certificate . bind (py) . str () ?, friendly_name_repr)) } }
};
}
