macro_rules! deps {
    () => {
        RepInterp!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < T : Iterator > Iterator for RepInterp < T > { type Item = T :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () } }
    };
}

impl_157!();