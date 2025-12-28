macro_rules! DefaultValue {
    () => {
        # [derive (Debug)] pub enum DefaultValue { Default , Value (Lit) , }
    };
}

DefaultValue!();