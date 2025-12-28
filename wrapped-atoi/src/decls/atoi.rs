macro_rules! deps {
    () => {
        FromRadix10SignedChecked!();
        FromRadix10!();
        FromRadix10Checked!();
        FromRadix10Signed!();
    };
}

macro_rules! atoi {
    () => {
        deps!();
        # [doc = " Parses an integer from a slice."] # [doc = ""] # [doc = " Contrary to its 'C' counterpart atoi is generic and will require a type argument if the type"] # [doc = " inference can not determine its result. It will also check for overflow / underflow and allow"] # [doc = " for Signs."] # [doc = ""] # [doc = " Use [`FromRadix10`] or [`FromRadix10Checked`] directly if you do not want to allow signs. Use"] # [doc = " [`FromRadix10`] or [`FromRadix10Signed`] if you want to opt out overflow / underflow checking."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use atoi::atoi;"] # [doc = " // Parsing to digits from a slice"] # [doc = " assert_eq!(Some(42), atoi::<u32>(b\"42\"));"] # [doc = " // Additional bytes after the number are ignored. If you want to know how many bytes were used"] # [doc = " // to parse the number use `FromRadix10::from_radix_10`."] # [doc = " assert_eq!(Some(42), atoi::<u32>(b\"42 is the answer to life, the universe and everything\"));"] # [doc = " // `None` is returned if the slice does not start with a digit"] # [doc = " assert_eq!(None, atoi::<u32>(b\"Sadly we do not know the question\"));"] # [doc = " // While signed integer types are supported..."] # [doc = " assert_eq!(Some(42), atoi::<i32>(b\"42\"));"] # [doc = " // Signs are allowed."] # [doc = " assert_eq!(Some(-42), atoi::<i32>(b\"-42\"));"] # [doc = " // Leading zeros are allowed"] # [doc = " assert_eq!(Some(42), atoi::<u32>(b\"0042\"));"] # [doc = " // Overflows will return `None`"] # [doc = " assert_eq!(None, atoi::<u8>(b\"256\"));"] # [doc = " ```"] # [doc = ""] # [doc = " # Return"] # [doc = ""] # [doc = " Returns a a number if the slice started with a number, otherwise `None` is returned."] pub fn atoi < I > (text : & [u8]) -> Option < I > where I : FromRadix10SignedChecked , { match I :: from_radix_10_signed_checked (text) { (_ , 0) | (None , _) => None , (Some (n) , _) => Some (n) , } }
    };
}

atoi!();