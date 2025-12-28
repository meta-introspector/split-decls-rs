macro_rules! deps {
    () => {
        Allow!();
        Error!();
    };
}

macro_rules! allow {
    () => {
        deps!();
        # [cfg (any (feature = "blocking-network-client" , feature = "async-network-client"))] mod allow { use std :: borrow :: Cow ; use crate :: { bstr :: BStr , config , config :: tree :: protocol :: Allow , remote :: url :: scheme_permission } ; impl Allow { # [doc = " Convert `value` into its respective `Allow` variant, possibly informing about the `scheme` we are looking at in the error."] pub fn try_into_allow (& 'static self , value : Cow < '_ , BStr > , scheme : Option < & str > ,) -> Result < scheme_permission :: Allow , config :: protocol :: allow :: Error > { scheme_permission :: Allow :: try_from (value) . map_err (| value | config :: protocol :: allow :: Error { value , scheme : scheme . map (ToOwned :: to_owned) , }) } } }
    };
}

allow!()