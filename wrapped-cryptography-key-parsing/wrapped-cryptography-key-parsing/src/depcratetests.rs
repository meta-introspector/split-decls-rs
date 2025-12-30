// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { KeyParsingError , KeySerializationError } ; # [test] fn test_key_parsing_error_from () { let e = openssl :: error :: ErrorStack :: get () ; assert ! (matches ! (KeyParsingError :: from (e) , KeyParsingError :: OpenSSL (_))) ; } # [test] fn test_key_serialization_error_from_asn1_write_error () { let e = asn1 :: WriteError :: AllocationError ; assert ! (matches ! (KeySerializationError :: from (e) , KeySerializationError :: Write (asn1 :: WriteError :: AllocationError))) ; } # [test] fn test_key_serialization_error_from_openssl_error_stack () { let e = openssl :: error :: ErrorStack :: get () ; assert ! (matches ! (KeySerializationError :: from (e) , KeySerializationError :: OpenSSL (_))) ; } }
};
}
