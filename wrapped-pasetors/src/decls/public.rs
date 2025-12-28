macro_rules! deps {
    () => {
        TrustedToken!();
        Error!();
        UntrustedToken!();
        PublicToken!();
        Claims!();
        AsymmetricSecretKey!();
        Footer!();
        AsymmetricPublicKey!();
        ClaimsValidationRules!();
        V4!();
        Public!();
    };
}

macro_rules! public {
    () => {
        deps!();
        # [cfg_attr (docsrs , doc (cfg (all (feature = "std" , feature = "v4"))))] # [cfg (all (feature = "std" , feature = "v4"))] # [doc = " PASETO public tokens with [`version4`], using [`claims::Claims`]."] pub mod public { use super :: * ; use crate :: claims :: { Claims , ClaimsValidationRules } ; use crate :: errors :: Error ; use crate :: footer :: Footer ; use crate :: keys :: { AsymmetricPublicKey , AsymmetricSecretKey } ; use crate :: token :: { TrustedToken , UntrustedToken } ; use crate :: version4 :: V4 ; # [doc = " Create a public token using the latest PASETO version (v4)."] pub fn sign (secret_key : & AsymmetricSecretKey < V4 > , message : & Claims , footer : Option < & Footer > , implicit_assert : Option < & [u8] > ,) -> Result < String , Error > { match footer { Some (f) => version4 :: PublicToken :: sign (secret_key , message . to_string () ? . as_bytes () , Some (f . to_string () ? . as_bytes ()) , implicit_assert ,) , None => version4 :: PublicToken :: sign (secret_key , message . to_string () ? . as_bytes () , None , implicit_assert ,) , } } # [doc = " Verify a public token using the latest PASETO version (v4). If verification passes,"] # [doc = " validate the claims according to the `validation_rules`."] pub fn verify (public_key : & AsymmetricPublicKey < V4 > , token : & UntrustedToken < Public , V4 > , validation_rules : & ClaimsValidationRules , footer : Option < & Footer > , implicit_assert : Option < & [u8] > ,) -> Result < TrustedToken , Error > { let mut trusted_token = match footer { Some (f) => version4 :: PublicToken :: verify (public_key , token , Some (f . to_string () ? . as_bytes ()) , implicit_assert ,) ? , None => version4 :: PublicToken :: verify (public_key , token , None , implicit_assert) ? , } ; let claims = Claims :: from_string (trusted_token . payload ()) ? ; validation_rules . validate_claims (& claims) ? ; trusted_token . set_payload_claims (claims) ; Ok (trusted_token) } }
    };
}

public!()