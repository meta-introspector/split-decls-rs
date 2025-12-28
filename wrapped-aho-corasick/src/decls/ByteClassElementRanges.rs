macro_rules! deps {
    () => {
        ByteClassElements!();
    };
}

macro_rules! ByteClassElementRanges {
    () => {
        deps!();
        # [doc = " An iterator over all elements in an equivalence class expressed as a"] # [doc = " sequence of contiguous ranges."] # [derive (Debug)] pub (crate) struct ByteClassElementRanges < 'a > { elements : ByteClassElements < 'a > , range : Option < (u8 , u8) > , }
    };
}

ByteClassElementRanges!();