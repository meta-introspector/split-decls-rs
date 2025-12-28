macro_rules! deps {
    () => {
        NodesFieldSwitcherSealed!();
        EnableNodesField!();
    };
}

macro_rules! impl_741 {
    () => {
        deps!();
        impl NodesFieldSwitcherSealed for EnableNodesField { }
    };
}

impl_741!()