macro_rules! deps {
    () => {
        StrConsumer!();
    };
}

macro_rules! Utf8SingleCodeUnitWriter {
    () => {
        deps!();
        # [doc = " A `Write` that only can handle bytes that are valid single-byte UTF-8 code units."] # [doc = ""] # [doc = " This is safe because we only use it when writing base64, which is always valid UTF-8."] struct Utf8SingleCodeUnitWriter < S : StrConsumer > { str_consumer : S , }
    };
}

Utf8SingleCodeUnitWriter!();