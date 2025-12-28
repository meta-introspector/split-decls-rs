macro_rules! Epoch {
    () => {
        # [doc = " An epoch that can be marked as pinned or unpinned."] # [doc = ""] # [doc = " Internally, the epoch is represented as an integer that wraps around at some unspecified point"] # [doc = " and a flag that represents whether it is pinned or unpinned."] # [derive (Copy , Clone , Default , Debug , Eq , PartialEq)] pub (crate) struct Epoch { # [doc = " The least significant bit is set if pinned. The rest of the bits hold the epoch."] data : usize , }
    };
}

Epoch!()