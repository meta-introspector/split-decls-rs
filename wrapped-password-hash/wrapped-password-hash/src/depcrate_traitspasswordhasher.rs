// Generated macro for PasswordHasher (trait)
macro_rules! Depcrate_traitsPasswordHasher {
() => {
// Module: crate::traits
// Provides: {"PasswordHasher"}
// Dependencies: {}
# [doc = " Trait for password hashing functions."] pub trait PasswordHasher { # [doc = " Algorithm-specific parameters."] type Params : Clone + Debug + Default + for < 'a > TryFrom < & 'a PasswordHash < 'a > , Error = Error > + TryInto < ParamsString , Error = Error > ; # [doc = " Compute a [`PasswordHash`] from the provided password using an"] # [doc = " explicit set of customized algorithm parameters as opposed to the"] # [doc = " defaults."] # [doc = ""] # [doc = " When in doubt, use [`PasswordHasher::hash_password`] instead."] fn hash_password_customized < 'a > (& self , password : & [u8] , algorithm : Option < Ident < 'a > > , version : Option < Decimal > , params : Self :: Params , salt : impl Into < Salt < 'a > > ,) -> Result < PasswordHash < 'a > > ; # [doc = " Simple API for computing a [`PasswordHash`] from a password and"] # [doc = " salt value."] # [doc = ""] # [doc = " Uses the default recommended parameters for a given algorithm."] fn hash_password < 'a > (& self , password : & [u8] , salt : impl Into < Salt < 'a > > ,) -> Result < PasswordHash < 'a > > { self . hash_password_customized (password , None , None , Self :: Params :: default () , salt) } }
};
}
