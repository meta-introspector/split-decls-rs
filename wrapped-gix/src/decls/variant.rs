macro_rules! deps {
    () => {
        Variant!();
        Ssh!();
    };
}

macro_rules! variant {
    () => {
        deps!();
        # [cfg (feature = "blocking-network-client")] mod variant { use std :: borrow :: Cow ; use crate :: { bstr :: BStr , config , config :: tree :: ssh :: Variant } ; impl Variant { pub fn try_into_variant (& 'static self , value : Cow < '_ , BStr > ,) -> Result < Option < gix_protocol :: transport :: client :: blocking_io :: ssh :: ProgramKind > , config :: key :: GenericErrorWithValue , > { use gix_protocol :: transport :: client :: blocking_io :: ssh :: ProgramKind ; use crate :: bstr :: ByteSlice ; Ok (Some (match value . as_ref () . as_bytes () { b"auto" => return Ok (None) , b"ssh" => ProgramKind :: Ssh , b"plink" => ProgramKind :: Plink , b"putty" => ProgramKind :: Putty , b"tortoiseplink" => ProgramKind :: TortoisePlink , b"simple" => ProgramKind :: Simple , _ => return Err (config :: key :: GenericErrorWithValue :: from_value (self , value . into_owned ())) , })) } } }
    };
}

variant!();