macro_rules! deps {
    () => {
        Regex!();
        Captures!();
    };
}

macro_rules! CaptureLocations {
    () => {
        deps!();
        # [doc = " A low level representation of the byte offsets of each capture group."] # [doc = ""] # [doc = " You can think of this as a lower level [`Captures`], where this type does"] # [doc = " not support named capturing groups directly and it does not borrow the"] # [doc = " haystack that these offsets were matched on."] # [doc = ""] # [doc = " Primarily, this type is useful when using the lower level `Regex` APIs such"] # [doc = " as [`Regex::captures_read`], which permits amortizing the allocation in"] # [doc = " which capture match offsets are stored."] # [doc = ""] # [doc = " In order to build a value of this type, you'll need to call the"] # [doc = " [`Regex::capture_locations`] method. The value returned can then be reused"] # [doc = " in subsequent searches for that regex. Using it for other regexes may"] # [doc = " result in a panic or otherwise incorrect results."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This example shows how to create and use `CaptureLocations` in a search."] # [doc = ""] # [doc = " ```"] # [doc = " use regex::Regex;"] # [doc = ""] # [doc = " let re = Regex::new(r\"(?<first>\\w+)\\s+(?<last>\\w+)\").unwrap();"] # [doc = " let mut locs = re.capture_locations();"] # [doc = " let m = re.captures_read(&mut locs, \"Bruce Springsteen\").unwrap();"] # [doc = " assert_eq!(0..17, m.range());"] # [doc = " assert_eq!(Some((0, 17)), locs.get(0));"] # [doc = " assert_eq!(Some((0, 5)), locs.get(1));"] # [doc = " assert_eq!(Some((6, 17)), locs.get(2));"] # [doc = ""] # [doc = " // Asking for an invalid capture group always returns None."] # [doc = " assert_eq!(None, locs.get(3));"] # [doc = " # // literals are too big for 32-bit usize: #1041"] # [doc = " # #[cfg(target_pointer_width = \"64\")]"] # [doc = " assert_eq!(None, locs.get(34973498648));"] # [doc = " # #[cfg(target_pointer_width = \"64\")]"] # [doc = " assert_eq!(None, locs.get(9944060567225171988));"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct CaptureLocations (captures :: Captures) ;
    };
}

CaptureLocations!()