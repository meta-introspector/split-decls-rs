// Generated macro for parse_general_name (function)
macro_rules! Depcrate_x509_commonparse_general_name {
() => {
// Module: crate::x509::common
// Provides: {"parse_general_name"}
// Dependencies: {}
pub (crate) fn parse_general_name < 'p > (py : pyo3 :: Python < 'p > , gn : GeneralName < '_ > ,) -> CryptographyResult < pyo3 :: Bound < 'p , pyo3 :: PyAny > > { let py_gn = match gn { GeneralName :: OtherName (data) => { let oid = oid_to_py_oid (py , & data . type_id) ? ; types :: OTHER_NAME . get (py) ? . call1 ((oid , data . value . full_data ())) ? } GeneralName :: RFC822Name (data) => types :: RFC822_NAME . get (py) ? . call_method1 (pyo3 :: intern ! (py , "_init_without_validation") , (data . 0 ,)) ? , GeneralName :: DNSName (data) => types :: DNS_NAME . get (py) ? . call_method1 (pyo3 :: intern ! (py , "_init_without_validation") , (data . 0 ,)) ? , GeneralName :: DirectoryName (data) => { let py_name = parse_name (py , data . unwrap_read ()) ? ; types :: DIRECTORY_NAME . get (py) ? . call1 ((py_name ,)) ? } GeneralName :: UniformResourceIdentifier (data) => types :: UNIFORM_RESOURCE_IDENTIFIER . get (py) ? . call_method1 (pyo3 :: intern ! (py , "_init_without_validation") , (data . 0 ,)) ? , GeneralName :: IPAddress (data) => { if data . len () == 4 || data . len () == 16 { let addr = types :: IPADDRESS_IPADDRESS . get (py) ? . call1 ((data ,)) ? ; types :: IP_ADDRESS . get (py) ? . call1 ((addr ,)) ? } else { create_ip_network (py , data) ? } } GeneralName :: RegisteredID (data) => { let oid = oid_to_py_oid (py , & data) ? ; types :: REGISTERED_ID . get (py) ? . call1 ((oid ,)) ? } _ => { return Err (CryptographyError :: from (exceptions :: UnsupportedGeneralNameType :: new_err ("x400Address/EDIPartyName are not supported types" ,) ,)) } } ; Ok (py_gn) }
};
}
