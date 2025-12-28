macro_rules! OwnerId {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Hash , Encodable , Decodable)] pub struct OwnerId { pub def_id : LocalDefId , }
    };
}

OwnerId!()