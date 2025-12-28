macro_rules! deps {
    () => {
        OidStat!();
    };
}

macro_rules! decode_oid_stat {
    () => {
        deps!();
        fn decode_oid_stat (data : & [u8] , hash_len : usize) -> Option < (OidStat , & [u8]) > { let (stat , data) = crate :: decode :: stat (data) ? ; let (hash , data) = data . split_at_checked (hash_len) ? ; Some ((OidStat { stat , id : ObjectId :: from_bytes_or_panic (hash) , } , data ,)) }
    };
}

decode_oid_stat!()