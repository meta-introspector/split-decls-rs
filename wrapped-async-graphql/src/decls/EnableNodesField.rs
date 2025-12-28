macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! EnableNodesField {
    () => {
        deps!();
        # [doc = " Enable (at compile time) `nodes` field in GQL output of [`Connection`]"] pub struct EnableNodesField ;
    };
}

EnableNodesField!()