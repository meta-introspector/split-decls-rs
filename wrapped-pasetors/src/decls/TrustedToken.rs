macro_rules! deps {
    () => {
        Claims!();
    };
}

macro_rules! TrustedToken {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] # [doc = " A [`TrustedToken`] is returned by either a `verify()` or `decrypt()` operation and represents"] # [doc = " a validated token."] # [doc = ""] # [doc = " It represents a authenticated and non-tampered token. It **does not** validate additional things,"] # [doc = " such as claims that may be within the token payload itself. These must still be validated separately."] # [doc = ""] # [doc = " However, using the [`crate::public`] and [`crate::local`] API will automatically handle claims"] # [doc = " validation. Any validated claims may be retrieved with [`TrustedToken::payload_claims()`]."] pub struct TrustedToken { header : String , payload : String , # [cfg (feature = "std")] # [cfg_attr (feature = "serde" , serde (default))] payload_claims : Option < Claims > , # [cfg_attr (feature = "serde" , serde (skip_serializing_if = "Vec::is_empty" , default))] footer : Vec < u8 > , # [cfg_attr (feature = "serde" , serde (skip_serializing_if = "Vec::is_empty" , default))] implicit_assert : Vec < u8 > , }
    };
}

TrustedToken!();