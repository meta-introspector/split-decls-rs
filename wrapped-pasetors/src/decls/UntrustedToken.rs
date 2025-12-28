macro_rules! UntrustedToken {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] # [doc = " [`UntrustedToken`] can parse PASETO tokens in order to extract individual parts of it."] # [doc = ""] # [doc = " A use-case for this would be parsing the tokens footer, if this is not known before receiving it. Then,"] # [doc = " the footer can be used during verification/decryption of the token itself."] # [doc = ""] # [doc = " This type should only be used in order to verify the validity of a token."] # [doc = ""] # [doc = " __WARNING__: Anything returned by this type should be treated as **UNTRUSTED** until the token"] # [doc = " has been verified."] pub struct UntrustedToken < T , V > { message : Vec < u8 > , footer : Vec < u8 > , # [cfg_attr (feature = "serde" , serde (skip))] phantom_t : PhantomData < T > , # [cfg_attr (feature = "serde" , serde (skip))] phantom_v : PhantomData < V > , }
    };
}

UntrustedToken!();