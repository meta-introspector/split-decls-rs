macro_rules! deps {
    () => {
        DisableNodesField!();
        EnableNodesField!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { pub trait NodesFieldSwitcher : Send + Sync { } impl NodesFieldSwitcher for super :: DisableNodesField { } impl NodesFieldSwitcher for super :: EnableNodesField { } }
    };
}

private!();