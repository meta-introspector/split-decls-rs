macro_rules! deps {
    () => {
        Response!();
        OnUpgrade!();
    };
}

macro_rules! sealed {
    () => {
        deps!();
        mod sealed { use super :: OnUpgrade ; pub trait CanUpgrade { fn on_upgrade (self) -> OnUpgrade ; } impl < B > CanUpgrade for http :: Request < B > { fn on_upgrade (mut self) -> OnUpgrade { self . extensions_mut () . remove :: < OnUpgrade > () . unwrap_or_else (OnUpgrade :: none) } } impl < B > CanUpgrade for & '_ mut http :: Request < B > { fn on_upgrade (self) -> OnUpgrade { self . extensions_mut () . remove :: < OnUpgrade > () . unwrap_or_else (OnUpgrade :: none) } } impl < B > CanUpgrade for http :: Response < B > { fn on_upgrade (mut self) -> OnUpgrade { self . extensions_mut () . remove :: < OnUpgrade > () . unwrap_or_else (OnUpgrade :: none) } } impl < B > CanUpgrade for & '_ mut http :: Response < B > { fn on_upgrade (self) -> OnUpgrade { self . extensions_mut () . remove :: < OnUpgrade > () . unwrap_or_else (OnUpgrade :: none) } } }
    };
}

sealed!()