macro_rules! Error {
    () => {
        # [doc = " The error used in the [`PartialNameRef`]`::try_from`(…) implementations."] pub type Error = gix_validate :: reference :: name :: Error ;
    };
}

Error!()