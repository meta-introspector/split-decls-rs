macro_rules! Bah {
    () => {
        # [doc = " A trait implemented by a Struct we want to mock"] pub trait Bah { # [doc = " Some trait method"] fn bah (& self) ; }
    };
}

Bah!();