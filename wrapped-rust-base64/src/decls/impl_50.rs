macro_rules! deps {
    () => {
        Engine!();
        Utf8SingleCodeUnitWriter!();
        StrConsumer!();
        EncoderStringWriter!();
        EncoderWriter!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < 'e , E : Engine , S : StrConsumer > EncoderStringWriter < 'e , E , S > { # [doc = " Create a `EncoderStringWriter` that will append to the provided `StrConsumer`."] pub fn from_consumer (str_consumer : S , engine : & 'e E) -> Self { EncoderStringWriter { encoder : EncoderWriter :: new (Utf8SingleCodeUnitWriter { str_consumer } , engine) , } } # [doc = " Encode all remaining buffered data, including any trailing incomplete input triples and"] # [doc = " associated padding."] # [doc = ""] # [doc = " Returns the base64-encoded form of the accumulated written data."] pub fn into_inner (mut self) -> S { self . encoder . finish () . expect ("Writing to a consumer should never fail") . str_consumer } }
    };
}

impl_50!();