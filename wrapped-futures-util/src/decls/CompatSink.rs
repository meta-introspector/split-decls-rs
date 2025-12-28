macro_rules! CompatSink {
    () => {
        # [doc = " Converts a futures 0.3 [`Sink`](futures_sink::Sink) into a futures 0.1"] # [doc = " [`Sink`](futures_01::sink::Sink)."] # [cfg (feature = "sink")] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] pub struct CompatSink < T , Item > { inner : T , _phantom : PhantomData < fn (Item) > , }
    };
}

CompatSink!();