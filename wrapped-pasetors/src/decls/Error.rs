macro_rules! deps {
    () => {
        Footer!();
        ClaimValidationError!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] # [doc = " Errors for token operations."] pub enum Error { # [doc = " Error for a token with an invalid format."] TokenFormat , # [doc = " Error for a failed Base64 (URL-safe without padding) encoding/decoding."] Base64 , # [doc = " Error for a failed token validation."] TokenValidation , # [doc = " Error for an invalid key."] Key , # [doc = " Error for a failed encryption operation."] Encryption , # [doc = " Error for a failed attempt to generate bytes using a CSPRNG."] Csprng , # [doc = " Error for a conversion that would be lossy."] LossyConversion , # [doc = " Error for attempting to create a token with an empty payload."] EmptyPayload , # [doc = " Error for attempting to create an invalid claim."] InvalidClaim , # [doc = " Claim validation error. See [`crate::claims::ClaimsValidationRules::validate_claims`]."] ClaimValidation (ClaimValidationError) , # [doc = " Error for attempting to parse a Claim but found invalid UTF-8 sequence."] ClaimInvalidUtf8 , # [doc = " Error for attempting to parse a Claim but found invalid JSON sequence."] ClaimInvalidJson , # [doc = " Error during (de)serialization of PASERK types."] PaserkParsing , # [doc = " Error during signing of a message."] Signing , # [doc = " Error during conversion between uncompressed<->compressed public keys."] PublicKeyConversion , # [doc = " Error during key generation."] KeyGeneration , # [doc = " The payload was not valid UTF-8."] PayloadInvalidUtf8 , # [doc = " Error during parsing of a `Footer`."] FooterParsing , }
    };
}

Error!()