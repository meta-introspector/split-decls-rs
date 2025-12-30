// Generated macro for custom_extensions (function)
macro_rules! Depcrate_ssl_testcustom_extensions {
() => {
// Module: crate::ssl::test
// Provides: {"custom_extensions"}
// Dependencies: {}
# [test] # [cfg (ossl111)] fn custom_extensions () { static FOUND_EXTENSION : AtomicBool = AtomicBool :: new (false) ; let mut server = Server :: builder () ; server . ctx () . add_custom_ext (12345 , ExtensionContext :: CLIENT_HELLO , | _ , _ , _ | -> Result < Option < & 'static [u8] > , _ > { unreachable ! () } , | _ , _ , data , _ | { FOUND_EXTENSION . store (data == b"hello" , Ordering :: SeqCst) ; Ok (()) } ,) . unwrap () ; let server = server . build () ; let mut client = server . client () ; client . ctx () . add_custom_ext (12345 , ssl :: ExtensionContext :: CLIENT_HELLO , | _ , _ , _ | Ok (Some (b"hello")) , | _ , _ , _ , _ | unreachable ! () ,) . unwrap () ; client . connect () ; assert ! (FOUND_EXTENSION . load (Ordering :: SeqCst)) ; }
};
}
