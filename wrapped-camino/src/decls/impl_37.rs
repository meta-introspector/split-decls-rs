macro_rules! deps {
    () => {
        Utf8Components!();
        Utf8Path!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < 'a > Utf8Components < 'a > { # [doc = " Extracts a slice corresponding to the portion of the path remaining for iteration."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use camino::Utf8Path;"] # [doc = ""] # [doc = " let mut components = Utf8Path::new(\"/tmp/foo/bar.txt\").components();"] # [doc = " components.next();"] # [doc = " components.next();"] # [doc = ""] # [doc = " assert_eq!(Utf8Path::new(\"foo/bar.txt\"), components.as_path());"] # [doc = " ```"] # [must_use] # [inline] pub fn as_path (& self) -> & 'a Utf8Path { unsafe { Utf8Path :: assume_utf8 (self . 0 . as_path ()) } } }
    };
}

impl_37!()