// Generated macro for clone_dh (function)
macro_rules! Depcrate_backend_dhclone_dh {
() => {
// Module: crate::backend::dh
// Provides: {"clone_dh"}
// Dependencies: {}
fn clone_dh < T : openssl :: pkey :: HasParams > (dh : & openssl :: dh :: Dh < T > ,) -> CryptographyResult < openssl :: dh :: Dh < openssl :: pkey :: Params > > { let p = dh . prime_p () . to_owned () ? ; let q = dh . prime_q () . map (| q | q . to_owned ()) . transpose () ? ; let g = dh . generator () . to_owned () ? ; Ok (openssl :: dh :: Dh :: from_pqg (p , q , g) ?) }
};
}
