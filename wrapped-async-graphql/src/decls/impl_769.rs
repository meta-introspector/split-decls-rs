macro_rules! deps {
    () => {
        Error!();
        ID!();
        Result!();
    };
}

macro_rules! impl_769 {
    () => {
        deps!();
        # [cfg (feature = "uuid")] impl TryFrom < ID > for uuid :: Uuid { type Error = uuid :: Error ; fn try_from (id : ID) -> Result < Self , Self :: Error > { uuid :: Uuid :: parse_str (& id . 0) } }
    };
}

impl_769!()