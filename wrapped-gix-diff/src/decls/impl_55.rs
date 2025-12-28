macro_rules! deps {
    () => {
        Recorder!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        # [doc = " Access"] impl Recorder { # [doc = " Obtain a copy of the currently tracked, full path of the entry."] pub fn path_clone (& self) -> BString { self . path . clone () } # [doc = " Return the currently set path."] pub fn path (& self) -> & BStr { self . path . as_ref () } }
    };
}

impl_55!();