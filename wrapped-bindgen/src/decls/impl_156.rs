macro_rules! deps {
    () => {
        RepInterp!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < T > RepInterp < T > { # [expect (clippy :: unnecessary_wraps)] pub fn next (self) -> Option < T > { Some (self . 0) } }
    };
}

impl_156!()