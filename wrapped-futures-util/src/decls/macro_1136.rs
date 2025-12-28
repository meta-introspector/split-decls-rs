macro_rules! deps {
    () => {
        Block!();
        Sink!();
    };
}

macro_rules! macro_1136 {
    () => {
        deps!();
        pin_project ! { # [doc = " Sink for the [`into_sink`](super::AsyncWriteExt::into_sink) method."] # [must_use = "sinks do nothing unless polled"] # [derive (Debug)] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] pub struct IntoSink < W , Item > { # [pin] writer : W , buffer : Option < Block < Item >>, } }
    };
}

macro_1136!()