macro_rules! deps {
    () => {
        Iso2022JpEncoderState!();
    };
}

macro_rules! Iso2022JpEncoder {
    () => {
        deps!();
        pub struct Iso2022JpEncoder { state : Iso2022JpEncoderState , }
    };
}

Iso2022JpEncoder!()