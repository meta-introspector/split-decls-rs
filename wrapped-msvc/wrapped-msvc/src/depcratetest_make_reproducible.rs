// Generated macro for test_make_reproducible (function)
macro_rules! Depcratetest_make_reproducible {
() => {
// Module: crate
// Provides: {"test_make_reproducible"}
// Dependencies: {}
# [test] fn test_make_reproducible () { for (machine , offset) in [("x86" , 0) , ("x64" , 12) , ("arm64" , 12)] { let mut def = std :: fs :: File :: create ("test.def") . unwrap () ; def . write_all (r#"
LIBRARY long-library-name-for-placement-in-long-names-table.dll
EXPORTS
A=A.#1
B=B.#2
C=C.#3
"# . to_string () . as_bytes () ,) . unwrap () ; drop (def) ; let mut cmd = std :: process :: Command :: new ("lib") ; cmd . arg ("/nologo") ; cmd . arg ("/Brepro") ; cmd . arg (format ! ("/machine:{machine}")) ; cmd . arg ("/out:test.lib") ; cmd . arg ("/def:test.def") ; cmd . output () . unwrap () ; let mut archive = std :: fs :: File :: options () . read (true) . write (true) . open ("test.lib") . unwrap () ; let mut buf = [0 ; 24] ; make_reproducible (std :: path :: Path :: new ("test.lib")) ; archive . seek (SeekFrom :: Start (0x18)) . unwrap () ; archive . read_exact (& mut buf [.. 12]) . unwrap () ; assert_eq ! (& buf [.. 12] , b"-1          ") ; archive . seek (SeekFrom :: Start (0x322 - offset)) . unwrap () ; archive . read_exact (& mut buf [.. 4]) . unwrap () ; assert_eq ! (& buf [.. 4] , [0x00 , 0x00 , 0x00 , 0x00]) ; archive . seek (SeekFrom :: Start (0x481 - offset)) . unwrap () ; archive . read_exact (& mut buf [.. 12]) . unwrap () ; assert_eq ! (& buf [.. 12] , b"@comp.id\0\0\0\0") ; archive . seek (SeekFrom :: Start (0x493 - offset)) . unwrap () ; archive . read_exact (& mut buf [.. 4]) . unwrap () ; assert_eq ! (& buf [.. 4] , [0x00 , 0x00 , 0x00 , 0x00]) ; archive . seek (SeekFrom :: Start (0xB4)) . unwrap () ; archive . read_exact (& mut buf [.. 24]) . unwrap () ; assert_eq ! (& buf [.. 24] , b"__NULL_IMPORT_DESCRIPTOR") ; archive . seek (SeekFrom :: Start (0x26E - offset)) . unwrap () ; archive . read_exact (& mut buf [.. 4]) . unwrap () ; assert_eq ! (& buf [.. 4] , b"//  ") ; archive . seek (SeekFrom :: Start (0x405 - offset)) . unwrap () ; archive . read_exact (& mut buf [.. 18]) . unwrap () ; assert_eq ! (& buf [.. 18] , b"Microsoft (R) LINK") ; drop (archive) ; std :: fs :: remove_file ("test.lib") . unwrap () ; std :: fs :: remove_file ("test.exp") . unwrap () ; std :: fs :: remove_file ("test.def") . unwrap () ; } }
};
}
