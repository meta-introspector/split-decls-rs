// Generated macro for parse_public_key (function)
macro_rules! Depcrate_rsaparse_public_key {
() => {
// Module: crate::rsa
// Provides: {"parse_public_key"}
// Dependencies: {}
fn parse_public_key (input : untrusted :: Input < '_ > ,) -> Result < PublicKeyComponents < & [u8] > , error :: Unspecified > { input . read_all (error :: Unspecified , | input | { der :: nested (input , der :: Tag :: Sequence , error :: Unspecified , | input | { let n = der :: nonnegative_integer (input) ? . as_slice_less_safe () ; let e = der :: nonnegative_integer (input) ? . as_slice_less_safe () ; Ok (PublicKeyComponents { n , e }) }) }) }
};
}
