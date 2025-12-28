macro_rules! deps {
    () => {
        Error!();
        Result!();
        ID!();
    };
}

macro_rules! impl_770 {
    () => {
        deps!();
        # [cfg (feature = "bson")] impl TryFrom < ID > for ObjectId { type Error = oid :: Error ; fn try_from (id : ID) -> std :: result :: Result < Self , oid :: Error > { ObjectId :: parse_str (id . 0) } }
    };
}

impl_770!()