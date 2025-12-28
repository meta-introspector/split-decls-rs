macro_rules! deps {
    () => {
        BiLock!();
        Sink!();
    };
}

macro_rules! SplitSink {
    () => {
        deps!();
        # [doc = " A `Sink` part of the split pair"] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] pub struct SplitSink < S , Item > { lock : BiLock < S > , slot : Option < Item > , }
    };
}

SplitSink!();