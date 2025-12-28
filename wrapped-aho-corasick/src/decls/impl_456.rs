macro_rules! deps {
    () => {
        Anchored!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        impl Anchored { # [doc = " Returns true if and only if this anchor mode corresponds to an anchored"] # [doc = " search."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use aho_corasick::Anchored;"] # [doc = ""] # [doc = " assert!(!Anchored::No.is_anchored());"] # [doc = " assert!(Anchored::Yes.is_anchored());"] # [doc = " ```"] # [inline] pub fn is_anchored (& self) -> bool { matches ! (* self , Anchored :: Yes) } }
    };
}

impl_456!();