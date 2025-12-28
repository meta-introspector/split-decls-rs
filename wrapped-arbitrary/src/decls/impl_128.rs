macro_rules! deps {
    () => {
        Arbitrary!();
        Result!();
        Unstructured!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for PathBuf { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < OsString as Arbitrary > :: arbitrary (u) . map (From :: from) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < OsString as Arbitrary > :: size_hint (depth) } }
    };
}

impl_128!();