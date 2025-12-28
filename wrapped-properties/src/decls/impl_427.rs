macro_rules! deps {
    () => {
        CodePointMapDataBorrowed!();
        BidiMirroringGlyph!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl MirroringFunc for CodePointMapDataBorrowed < '_ , BidiMirroringGlyph > { fn mirroring (& self , ch : char) -> char { self . get (ch) . mirroring_glyph . unwrap_or (ch) } }
    };
}

impl_427!()