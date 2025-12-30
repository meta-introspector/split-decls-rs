// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [allow (clippy :: unwrap_used)] mod tests { use hex_literal :: hex ; use spki :: ObjectIdentifier ; use super :: * ; use der :: Decode ; # [test] fn initial_context_token () { let gss_bytes = hex ! ("604806062b0601050502a03e303ca00e300c060a2b06010401823702020aa22a04284e544c4d535350000100000005028862000000000000000000000000000000000601b01d0000000f") ; let inner_bytes = hex ! ("303ca00e300c060a2b06010401823702020aa22a04284e544c4d535350000100000005028862000000000000000000000000000000000601b01d0000000f") ; let gss = InitialContextToken :: from_der (& gss_bytes) . unwrap () ; assert_eq ! (ObjectIdentifier :: new_unwrap ("1.3.6.1.5.5.2") , gss . this_mech) ; assert_eq ! (AnyRef :: new (Tag :: ContextSpecific { constructed : true , number : TagNumber (0) } , & inner_bytes) . unwrap () , gss . inner_context_token) ; let output = InitialContextToken { this_mech : MechType :: new_unwrap ("1.3.6.1.5.5.2") , inner_context_token : AnyRef :: new (Tag :: ContextSpecific { constructed : true , number : TagNumber (0) , } , & inner_bytes ,) . unwrap () , } ; let output_bytes = output . to_der () . unwrap () ; assert_eq ! (& gss_bytes [..] , & output_bytes) ; } }
};
}
