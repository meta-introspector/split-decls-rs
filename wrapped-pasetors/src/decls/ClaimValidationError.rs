macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ClaimValidationError {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] # [doc = " Errors for claim validation operations."] pub enum ClaimValidationError { # [doc = " `Audience` claim doesn't match."] Aud , # [doc = " `Expiration` claim expired."] Exp , # [doc = " `Issued at` claim has date set in the future."] Iat , # [doc = " `Issuer` claim doesn't match."] Iss , # [doc = " `Token Identifier` doesn't match."] Jti , # [doc = " `Not before` claim date not reached."] Nbf , # [doc = " `Subject` claim doesn't match."] Sub , # [doc = " No `Audience` claim was set."] NoAud , # [doc = " No `Expiration` claim was set."] NoExp , # [doc = " No `Issued at` claim was set."] NoIat , # [doc = " No `Issuer` claim was set."] NoIss , # [doc = " No `Token Identifier` claim was set."] NoJti , # [doc = " No `Not before` claim was set."] NoNbf , # [doc = " No `Subject` claim was set."] NoSub , # [doc = " Claim `Expiration` is no string."] NoStrExp , # [doc = " Claim `Issued at` is no string."] NoStrIat , # [doc = " Claim `Not before` is no string."] NoStrNbf , # [doc = " Error during parsing of `Expiration` claim."] ParseExp , # [doc = " Error during parsing of `Issued at` claim."] ParseIat , # [doc = " Error during parsing of `Not before` claim."] ParseNbf , }
    };
}

ClaimValidationError!();