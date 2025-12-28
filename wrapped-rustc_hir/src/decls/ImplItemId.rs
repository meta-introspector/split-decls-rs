macro_rules! ImplItemId {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct ImplItemId { pub owner_id : OwnerId , }
    };
}

ImplItemId!()