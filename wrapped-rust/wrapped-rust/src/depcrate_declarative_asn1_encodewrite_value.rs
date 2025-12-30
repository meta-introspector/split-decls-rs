// Generated macro for write_value (function)
macro_rules! Depcrate_declarative_asn1_encodewrite_value {
() => {
// Module: crate::declarative_asn1::encode
// Provides: {"write_value"}
// Dependencies: {}
fn write_value < T : SimpleAsn1Writable > (writer : & mut Writer < '_ > , value : & T , encoding : & Option < pyo3 :: Py < Encoding > > ,) -> Result < () , asn1 :: WriteError > { match encoding { Some (e) => match e . get () { Encoding :: Implicit (tag) => writer . write_implicit_element (value , * tag) , Encoding :: Explicit (tag) => writer . write_explicit_element (value , * tag) , } , None => writer . write_element (value) , } }
};
}
