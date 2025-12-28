macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! UpgradeExpected {
    () => {
        deps!();
        # [doc = " Error cause returned when an upgrade was expected but canceled"] # [doc = " for whatever reason."] # [doc = ""] # [doc = " This likely means the actual `Conn` future wasn't polled and upgraded."] # [derive (Debug)] struct UpgradeExpected ;
    };
}

UpgradeExpected!();