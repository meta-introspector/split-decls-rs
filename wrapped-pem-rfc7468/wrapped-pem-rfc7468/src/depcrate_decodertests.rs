// Generated macro for tests (module)
macro_rules! Depcrate_decodertests {
() => {
// Module: crate::decoder
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [allow (clippy :: unwrap_used)] mod tests { use super :: Encapsulation ; # [test] fn pkcs8_example () { let pem = include_bytes ! ("../tests/examples/pkcs8.pem") ; let encapsulation = Encapsulation :: parse (pem) . unwrap () ; assert_eq ! (encapsulation . label , "PRIVATE KEY") ; assert_eq ! (encapsulation . encapsulated_text , & [77 , 67 , 52 , 67 , 65 , 81 , 65 , 119 , 66 , 81 , 89 , 68 , 75 , 50 , 86 , 119 , 66 , 67 , 73 , 69 , 73 , 66 , 102 , 116 , 110 , 72 , 80 , 112 , 50 , 50 , 83 , 101 , 119 , 89 , 109 , 109 , 69 , 111 , 77 , 99 , 88 , 56 , 86 , 119 , 73 , 52 , 73 , 72 , 119 , 97 , 113 , 100 , 43 , 57 , 76 , 70 , 80 , 106 , 47 , 49 , 53 , 101 , 113 , 70]) ; } }
};
}
