macro_rules! Id {
    () => {
        # [derive (Eq , PartialEq , Hash , Copy , Clone)] pub (crate) struct Id { execution_id : execution :: Id , id : usize , }
    };
}

Id!();