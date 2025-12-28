macro_rules! TokenAtOffset {
    () => {
        # [doc = " There might be zero, one or two leaves at a given offset."] # [derive (Clone , Debug)] pub enum TokenAtOffset < T > { # [doc = " No leaves at offset -- possible for the empty file."] None , # [doc = " Only a single leaf at offset."] Single (T) , # [doc = " Offset is exactly between two leaves."] Between (T , T) , }
    };
}

TokenAtOffset!()