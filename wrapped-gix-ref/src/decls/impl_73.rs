macro_rules! deps {
    () => {
        Target!();
        Error!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl TryFrom < Target > for ObjectId { type Error = Target ; fn try_from (value : Target) -> Result < Self , Self :: Error > { match value { Target :: Object (id) => Ok (id) , Target :: Symbolic (_) => Err (value) , } } }
    };
}

impl_73!();