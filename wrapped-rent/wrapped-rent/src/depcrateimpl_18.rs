// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
# [allow (deprecated)] impl RentDue { # [doc = " Return the lamports due for rent."] pub fn lamports (& self) -> u64 { match self { RentDue :: Exempt => 0 , RentDue :: Paying (x) => * x , } } # [doc = " Return 'true' if rent exempt."] pub fn is_exempt (& self) -> bool { match self { RentDue :: Exempt => true , RentDue :: Paying (_) => false , } } }
};
}
