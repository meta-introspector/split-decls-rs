macro_rules! deps {
    () => {
        CodePointMapData!();
        BidiMirroringGlyph!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl MirroringFunc for CodePointMapData < BidiMirroringGlyph > { fn mirroring (& self , ch : char) -> char { MirroringFunc :: mirroring (& self . as_borrowed () , ch) } }
    };
}

impl_428!()