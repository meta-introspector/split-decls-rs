// Generated macro for tests (module)
macro_rules! Depcrate_x509_scttests {
() => {
// Module: crate::x509::sct
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_hash_algorithm_try_from () { for (n , ha) in & [(1_u8 , HashAlgorithm :: Md5) , (2_u8 , HashAlgorithm :: Sha1) , (3_u8 , HashAlgorithm :: Sha224) , (4_u8 , HashAlgorithm :: Sha256) , (5_u8 , HashAlgorithm :: Sha384) , (6_u8 , HashAlgorithm :: Sha512) ,] { let res = HashAlgorithm :: try_from (* n) . unwrap () ; assert_eq ! (& res , ha) ; } assert ! (HashAlgorithm :: try_from (0) . is_err ()) ; assert ! (HashAlgorithm :: try_from (7) . is_err ()) ; } # [test] fn test_hash_algorithm_to_attr () { for (ha , attr) in & [(HashAlgorithm :: Md5 , "MD5") , (HashAlgorithm :: Sha1 , "SHA1") , (HashAlgorithm :: Sha224 , "SHA224") , (HashAlgorithm :: Sha256 , "SHA256") , (HashAlgorithm :: Sha384 , "SHA384") , (HashAlgorithm :: Sha512 , "SHA512") ,] { assert_eq ! (ha . to_attr () , * attr) ; } } # [test] fn test_signature_algorithm_try_from () { for (n , ha) in & [(1_u8 , SignatureAlgorithm :: Rsa) , (2_u8 , SignatureAlgorithm :: Dsa) , (3_u8 , SignatureAlgorithm :: Ecdsa) ,] { let res = SignatureAlgorithm :: try_from (* n) . unwrap () ; assert_eq ! (& res , ha) ; } assert ! (SignatureAlgorithm :: try_from (0) . is_err ()) ; assert ! (SignatureAlgorithm :: try_from (4) . is_err ()) ; } # [test] fn test_signature_algorithm_to_attr () { for (sa , attr) in & [(SignatureAlgorithm :: Rsa , "RSA") , (SignatureAlgorithm :: Dsa , "DSA") , (SignatureAlgorithm :: Ecdsa , "ECDSA") ,] { assert_eq ! (sa . to_attr () , * attr) ; } } }
};
}
