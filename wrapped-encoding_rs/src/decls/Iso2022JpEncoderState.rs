macro_rules! Iso2022JpEncoderState {
    () => {
        enum Iso2022JpEncoderState { Ascii , Roman , Jis0208 , }
    };
}

Iso2022JpEncoderState!()