macro_rules! deps {
    () => {
        Utf8Prefix!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl Utf8Prefix < '_ > { # [doc = " Determines if the prefix is verbatim, i.e., begins with `\\\\?\\`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use camino::Utf8Prefix::*;"] # [doc = ""] # [doc = " assert!(Verbatim(\"pictures\").is_verbatim());"] # [doc = " assert!(VerbatimUNC(\"server\", \"share\").is_verbatim());"] # [doc = " assert!(VerbatimDisk(b'C').is_verbatim());"] # [doc = " assert!(!DeviceNS(\"BrainInterface\").is_verbatim());"] # [doc = " assert!(!UNC(\"server\", \"share\").is_verbatim());"] # [doc = " assert!(!Disk(b'C').is_verbatim());"] # [doc = " ```"] # [must_use] pub fn is_verbatim (& self) -> bool { use Utf8Prefix :: * ; matches ! (self , Verbatim (_) | VerbatimDisk (_) | VerbatimUNC (..)) } }
    };
}

impl_65!();