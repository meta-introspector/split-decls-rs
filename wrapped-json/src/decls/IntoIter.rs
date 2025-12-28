macro_rules! deps {
    () => {
        IntoIterImpl!();
    };
}

macro_rules! IntoIter {
    () => {
        deps!();
        # [doc = " An owning iterator over a serde_json::Map's entries."] # [derive (Debug)] pub struct IntoIter { iter : IntoIterImpl , }
    };
}

IntoIter!()