macro_rules! deps {
    () => {
        U8!();
        Unit!();
    };
}

macro_rules! UnitKind {
    () => {
        deps!();
        # [derive (Clone , Copy , Eq , PartialEq , PartialOrd , Ord)] enum UnitKind { # [doc = " Represents a byte value, or more typically, an equivalence class"] # [doc = " represented as a byte value."] U8 (u8) , # [doc = " Represents the \"end of input\" sentinel. We regrettably use a `u16`"] # [doc = " here since the maximum sentinel value is `256`. Thankfully, we don't"] # [doc = " actually store a `Unit` anywhere, so this extra space shouldn't be too"] # [doc = " bad."] EOI (u16) , }
    };
}

UnitKind!();