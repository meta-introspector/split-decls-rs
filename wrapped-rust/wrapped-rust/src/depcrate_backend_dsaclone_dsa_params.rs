// Generated macro for clone_dsa_params (function)
macro_rules! Depcrate_backend_dsaclone_dsa_params {
() => {
// Module: crate::backend::dsa
// Provides: {"clone_dsa_params"}
// Dependencies: {}
fn clone_dsa_params < T : openssl :: pkey :: HasParams > (d : & openssl :: dsa :: Dsa < T > ,) -> Result < openssl :: dsa :: Dsa < openssl :: pkey :: Params > , openssl :: error :: ErrorStack > { openssl :: dsa :: Dsa :: from_pqg (d . p () . to_owned () ? , d . q () . to_owned () ? , d . g () . to_owned () ?) }
};
}
