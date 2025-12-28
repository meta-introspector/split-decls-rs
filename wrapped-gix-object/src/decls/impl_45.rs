macro_rules! deps {
    () => {
        Error!();
        CommitRef!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl < 'a > CommitRef < 'a > { # [doc = " Deserialize a commit from the given `data` bytes while avoiding most allocations."] pub fn from_bytes (mut data : & 'a [u8]) -> Result < CommitRef < 'a > , crate :: decode :: Error > { let input = & mut data ; match decode :: commit . parse_next (input) { Ok (tag) => Ok (tag) , Err (err) => Err (crate :: decode :: Error :: with_err (err , input)) , } } }
    };
}

impl_45!()