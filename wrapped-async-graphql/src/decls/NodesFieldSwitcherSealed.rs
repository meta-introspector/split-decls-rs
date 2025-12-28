macro_rules! deps {
    () => {
        Connection!();
    };
}

macro_rules! NodesFieldSwitcherSealed {
    () => {
        deps!();
        # [doc = " Allow switch if [`Connection`] contains `nodes` field in GQL output"] # [doc = ""] # [doc = " This trait is sealed and can not be implemented outside of this crate."] pub trait NodesFieldSwitcherSealed : private :: NodesFieldSwitcher { }
    };
}

NodesFieldSwitcherSealed!()