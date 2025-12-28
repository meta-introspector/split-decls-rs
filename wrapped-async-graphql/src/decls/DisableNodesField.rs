macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! DisableNodesField {
    () => {
        deps!();
        # [doc = " Disable (at compile time) `nodes` field in GQL output of [`Connection`]"] pub struct DisableNodesField ;
    };
}

DisableNodesField!()