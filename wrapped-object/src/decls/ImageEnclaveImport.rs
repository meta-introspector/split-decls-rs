macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageEnclaveImport {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageEnclaveImport { pub match_type : U32 < LE > , pub minimum_security_version : U32 < LE > , pub unique_or_author_id : [u8 ; IMAGE_ENCLAVE_LONG_ID_LENGTH] , pub family_id : [u8 ; IMAGE_ENCLAVE_SHORT_ID_LENGTH] , pub image_id : [u8 ; IMAGE_ENCLAVE_SHORT_ID_LENGTH] , pub import_name : U32 < LE > , pub reserved : U32 < LE > , }
    };
}

ImageEnclaveImport!()