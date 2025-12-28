macro_rules! deps {
    () => {
        Error!();
        IndexVersion!();
        Version!();
    };
}

macro_rules! index_version {
    () => {
        deps!();
        mod index_version { use crate :: { config , config :: tree :: sections :: pack :: IndexVersion } ; impl IndexVersion { # [doc = " Try to interpret an integer value as index version."] pub fn try_into_index_version (& 'static self , value : Result < i64 , gix_config :: value :: Error > ,) -> Result < gix_pack :: index :: Version , config :: key :: GenericError > { let value = value . map_err (| err | config :: key :: GenericError :: from (self) . with_source (err)) ? ; Ok (match value { 1 => gix_pack :: index :: Version :: V1 , 2 => gix_pack :: index :: Version :: V2 , _ => return Err (config :: key :: GenericError :: from (self)) , }) } } }
    };
}

index_version!()