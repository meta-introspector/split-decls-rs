macro_rules! deps {
    () => {
        Engine!();
        StrConsumer!();
        EncoderWriter!();
        Utf8SingleCodeUnitWriter!();
    };
}

macro_rules! EncoderStringWriter {
    () => {
        deps!();
        # [doc = " A `Write` implementation that base64-encodes data using the provided config and accumulates the"] # [doc = " resulting base64 utf8 `&str` in a [`StrConsumer`] implementation (typically `String`), which is"] # [doc = " then exposed via `into_inner()`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Buffer base64 in a new String:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::Write;"] # [doc = " use base64::engine::general_purpose;"] # [doc = ""] # [doc = " let mut enc = base64::write::EncoderStringWriter::new(&general_purpose::STANDARD);"] # [doc = ""] # [doc = " enc.write_all(b\"asdf\").unwrap();"] # [doc = ""] # [doc = " // get the resulting String"] # [doc = " let b64_string = enc.into_inner();"] # [doc = ""] # [doc = " assert_eq!(\"YXNkZg==\", &b64_string);"] # [doc = " ```"] # [doc = ""] # [doc = " Or, append to an existing `String`, which implements `StrConsumer`:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::io::Write;"] # [doc = " use base64::engine::general_purpose;"] # [doc = ""] # [doc = " let mut buf = String::from(\"base64: \");"] # [doc = ""] # [doc = " let mut enc = base64::write::EncoderStringWriter::from_consumer("] # [doc = "     &mut buf,"] # [doc = "     &general_purpose::STANDARD);"] # [doc = ""] # [doc = " enc.write_all(b\"asdf\").unwrap();"] # [doc = ""] # [doc = " // release the &mut reference on buf"] # [doc = " let _ = enc.into_inner();"] # [doc = ""] # [doc = " assert_eq!(\"base64: YXNkZg==\", &buf);"] # [doc = " ```"] # [doc = ""] # [doc = " # Performance"] # [doc = ""] # [doc = " Because it has to validate that the base64 is UTF-8, it is about 80% as fast as writing plain"] # [doc = " bytes to a `io::Write`."] pub struct EncoderStringWriter < 'e , E : Engine , S : StrConsumer > { encoder : EncoderWriter < 'e , E , Utf8SingleCodeUnitWriter < S > > , }
    };
}

EncoderStringWriter!()