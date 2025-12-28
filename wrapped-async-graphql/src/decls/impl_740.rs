macro_rules! deps {
    () => {
        DisableNodesField!();
        NodesFieldSwitcherSealed!();
    };
}

macro_rules! impl_740 {
    () => {
        deps!();
        impl NodesFieldSwitcherSealed for DisableNodesField { }
    };
}

impl_740!();