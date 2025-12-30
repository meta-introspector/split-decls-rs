// Generated macro for CollationSpecialPrimariesValidated (struct)
macro_rules! Depcrate_providerCollationSpecialPrimariesValidated {
() => {
// Module: crate::provider
// Provides: {"CollationSpecialPrimariesValidated"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] pub (crate) struct CollationSpecialPrimariesValidated < 'data > { # [doc = " The primaries corresponding to `MaxVariable`"] # [doc = " character classes packed so that each fits in"] # [doc = " 16 bits. Length must match the number of enum"] # [doc = " variants in `MaxVariable`, currently 4."] pub last_primaries : ZeroVec < 'data , u16 > , # [doc = " The high 8 bits of the numeric primary"] pub numeric_primary : u8 , # [doc = " 256 bits (packed in 16 u16s) to classify every possible"] # [doc = " byte into compressible or non-compressible."] pub compressible_bytes : & 'data [< u16 as AsULE > :: ULE ; 16] , }
};
}
