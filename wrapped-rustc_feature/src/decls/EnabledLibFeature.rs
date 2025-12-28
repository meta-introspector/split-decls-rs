macro_rules! EnabledLibFeature {
    () => {
        # [doc = " Information about an enabled library feature."] # [derive (Debug , Copy , Clone)] pub struct EnabledLibFeature { pub gate_name : Symbol , pub attr_sp : Span , }
    };
}

EnabledLibFeature!();