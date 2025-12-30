// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] # [allow (deprecated)] fn test_due () { let default_rent = Rent :: default () ; assert_eq ! (default_rent . due (0 , 2 , 1.2) , RentDue :: Paying ((((2 + ACCOUNT_STORAGE_OVERHEAD) * DEFAULT_LAMPORTS_PER_BYTE_YEAR) as f64 * 1.2) as u64) ,) ; assert_eq ! (default_rent . due ((((2 + ACCOUNT_STORAGE_OVERHEAD) * DEFAULT_LAMPORTS_PER_BYTE_YEAR) as f64 * DEFAULT_EXEMPTION_THRESHOLD) as u64 , 2 , 1.2) , RentDue :: Exempt ,) ; let custom_rent = Rent { lamports_per_byte_year : 5 , exemption_threshold : 2.5 , .. Rent :: default () } ; assert_eq ! (custom_rent . due (0 , 2 , 1.2) , RentDue :: Paying ((((2 + ACCOUNT_STORAGE_OVERHEAD) * custom_rent . lamports_per_byte_year) as f64 * 1.2) as u64 ,)) ; assert_eq ! (custom_rent . due ((((2 + ACCOUNT_STORAGE_OVERHEAD) * custom_rent . lamports_per_byte_year) as f64 * custom_rent . exemption_threshold) as u64 , 2 , 1.2) , RentDue :: Exempt) ; } # [test] # [allow (deprecated)] fn test_rent_due_lamports () { assert_eq ! (RentDue :: Exempt . lamports () , 0) ; let amount = 123 ; assert_eq ! (RentDue :: Paying (amount) . lamports () , amount) ; } # [test] # [allow (deprecated)] fn test_rent_due_is_exempt () { assert ! (RentDue :: Exempt . is_exempt ()) ; assert ! (! RentDue :: Paying (0) . is_exempt ()) ; } # [test] # [allow (deprecated)] fn test_clone () { let rent = Rent { lamports_per_byte_year : 1 , exemption_threshold : 2.2 , burn_percent : 3 , } ; # [allow (clippy :: clone_on_copy)] let cloned_rent = rent . clone () ; assert_eq ! (cloned_rent , rent) ; } }
};
}
