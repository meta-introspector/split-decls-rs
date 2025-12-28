macro_rules! deps {
    () => {
        EnableNodesField!();
        NodesFieldSwitcherSealed!();
    };
}

macro_rules! impl_741 {
    () => {
        deps!();
        impl NodesFieldSwitcherSealed for EnableNodesField { }
    };
}

impl_741!();