macro_rules! UnresolvedMacro {
    () => {
        # [derive (Debug)] pub struct UnresolvedMacro { pub path : ModPath , }
    };
}

UnresolvedMacro!();