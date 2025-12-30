// Generated macro for psk_ciphers (function)
macro_rules! Depcrate_ssl_testpsk_ciphers {
() => {
// Module: crate::ssl::test
// Provides: {"psk_ciphers"}
// Dependencies: {}
# [cfg (not (osslconf = "OPENSSL_NO_PSK"))] # [test] fn psk_ciphers () { const CIPHER : & str = "PSK-AES256-CBC-SHA" ; const PSK : & [u8] = b"thisisaverysecurekey" ; const CLIENT_IDENT : & [u8] = b"thisisaclient" ; static CLIENT_CALLED : AtomicBool = AtomicBool :: new (false) ; static SERVER_CALLED : AtomicBool = AtomicBool :: new (false) ; let mut server = Server :: builder () ; server . ctx () . set_cipher_list (CIPHER) . unwrap () ; server . ctx () . set_psk_server_callback (| _ , identity , psk | { assert ! (identity . unwrap_or (& []) == CLIENT_IDENT) ; psk [.. PSK . len ()] . copy_from_slice (PSK) ; SERVER_CALLED . store (true , Ordering :: SeqCst) ; Ok (PSK . len ()) }) ; let server = server . build () ; let mut client = server . client () ; # [cfg (any (boringssl , ossl111 , awslc))] client . ctx () . set_options (super :: SslOptions :: NO_TLSV1_3) ; client . ctx () . set_cipher_list (CIPHER) . unwrap () ; client . ctx () . set_psk_client_callback (move | _ , _ , identity , psk | { identity [.. CLIENT_IDENT . len ()] . copy_from_slice (CLIENT_IDENT) ; identity [CLIENT_IDENT . len ()] = 0 ; psk [.. PSK . len ()] . copy_from_slice (PSK) ; CLIENT_CALLED . store (true , Ordering :: SeqCst) ; Ok (PSK . len ()) }) ; client . connect () ; assert ! (SERVER_CALLED . load (Ordering :: SeqCst)) ; assert ! (CLIENT_CALLED . load (Ordering :: SeqCst)) ; }
};
}
