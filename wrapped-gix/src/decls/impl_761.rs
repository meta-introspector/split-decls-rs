macro_rules! deps {
    () => {
        Error!();
        Any!();
        String!();
        Validate!();
    };
}

macro_rules! impl_761 {
    () => {
        deps!();
        # [doc = " Conversion"] impl < T : Validate > Any < T > { # [doc = " Try to convert `value` into a refspec suitable for the `op` operation."] pub fn try_into_refspec (& 'static self , value : std :: borrow :: Cow < '_ , BStr > , op : gix_refspec :: parse :: Operation ,) -> Result < gix_refspec :: RefSpec , config :: refspec :: Error > { gix_refspec :: parse (value . as_ref () , op) . map (| spec | spec . to_owned ()) . map_err (| err | config :: refspec :: Error :: from_value (self , value . into_owned ()) . with_source (err)) } # [doc = " Try to interpret `value` as UTF-8 encoded string."] pub fn try_into_string (& 'static self , value : Cow < '_ , BStr >) -> Result < std :: string :: String , config :: string :: Error > { use crate :: bstr :: ByteVec ; Vec :: from (value . into_owned ()) . into_string () . map_err (| err | { let utf8_err = err . utf8_error () . clone () ; config :: string :: Error :: from_value (self , err . into_vec () . into ()) . with_source (utf8_err) }) } }
    };
}

impl_761!()