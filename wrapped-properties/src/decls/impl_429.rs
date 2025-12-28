macro_rules! deps {
    () => {
        BidiMirroringGlyph!();
        CodePointMapData!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl MirroringFunc for & '_ CodePointMapData < BidiMirroringGlyph > { fn mirroring (& self , ch : char) -> char { MirroringFunc :: mirroring (& self . as_borrowed () , ch) } }
    };
}

impl_429!()