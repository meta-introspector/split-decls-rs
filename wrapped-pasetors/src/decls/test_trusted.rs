macro_rules! deps {
    () => {
        TrustedToken!();
        Claims!();
        Generate!();
        ClaimsValidationRules!();
        SymmetricKey!();
        UntrustedToken!();
        Local!();
        Footer!();
        V4!();
    };
}

macro_rules! test_trusted {
    () => {
        deps!();
        # [cfg (all (test , feature = "serde" , feature = "v4"))] mod test_trusted { use super :: * ; use crate :: claims :: ClaimsValidationRules ; use crate :: keys :: { Generate , SymmetricKey } ; use crate :: local ; use crate :: version4 :: V4 ; # [test] # [doc = " Simple serialize -> deserialize test to ensure that round trip produces the same result"] fn test_serde () { let sk = SymmetricKey :: < V4 > :: generate () . unwrap () ; let mut claims = Claims :: new () . unwrap () ; claims . add_additional ("test" , "serde") . unwrap () ; let mut footer = Footer :: default () ; footer . add_additional ("test" , "footer") . unwrap () ; let token = local :: encrypt (& sk , & claims , Some (& footer) , None) . unwrap () ; let validation_rules = ClaimsValidationRules :: default () ; let untrusted = UntrustedToken :: < Local , V4 > :: try_from (& token) . unwrap () ; assert_eq ! (& untrusted , & serde_json :: from_str ::< UntrustedToken < Local , V4 >> (& serde_json :: to_string (& untrusted) . unwrap ()) . unwrap ()) ; let trusted = local :: decrypt (& sk , & untrusted , & validation_rules , Some (& footer) , None) . unwrap () ; let json = serde_json :: to_string (& trusted) . unwrap () ; assert ! (json . contains ("\"payload\"")) ; assert ! (json . contains ("\"payload_claims\"")) ; assert ! (json . contains ("\\\"test\\\":\\\"serde\\\"")) ; assert ! (json . contains ("\"footer\":[123,34,116,101,115,116,34,58,34,102,111,111,116,101,114,34,125]")) ; assert ! (! json . contains ("implicit_assert")) ; let output : TrustedToken = serde_json :: from_str (& json) . unwrap () ; assert_eq ! (output , trusted) ; assert_eq ! (output . payload_claims . unwrap () , claims) ; } }
    };
}

test_trusted!()