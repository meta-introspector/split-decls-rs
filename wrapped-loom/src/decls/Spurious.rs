macro_rules! Spurious {
    () => {
        # [derive (Debug)] # [cfg_attr (feature = "checkpoint" , derive (Serialize , Deserialize))] pub (crate) struct Spurious { spur : bool , exploring : bool , }
    };
}

Spurious!();