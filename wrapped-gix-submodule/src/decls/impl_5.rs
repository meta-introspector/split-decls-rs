macro_rules! deps {
    () => {
        Ignore!();
        Error!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl TryFrom < & BStr > for Ignore { type Error = () ; fn try_from (value : & BStr) -> Result < Self , Self :: Error > { Ok (match value . as_bytes () { b"all" => Ignore :: All , b"dirty" => Ignore :: Dirty , b"untracked" => Ignore :: Untracked , b"none" => Ignore :: None , _ => return Err (()) , }) } }
    };
}

impl_5!()