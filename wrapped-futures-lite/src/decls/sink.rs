macro_rules! deps {
    () => {
        AsyncWriteExt!();
        Sink!();
    };
}

macro_rules! sink {
    () => {
        deps!();
        # [doc = " Creates a writer that consumes and drops all data."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{self, AsyncWriteExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut writer = io::sink();"] # [doc = " writer.write_all(b\"hello\").await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub fn sink () -> Sink { Sink { _private : () } }
    };
}

sink!()