macro_rules! Iso2022JpDecoderState {
    () => {
        # [derive (Copy , Clone , PartialEq)] enum Iso2022JpDecoderState { Ascii , Roman , Katakana , LeadByte , TrailByte , EscapeStart , Escape , }
    };
}

Iso2022JpDecoderState!();