macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! Compat01As03Sink {
    () => {
        deps!();
        # [doc = " Converts a futures 0.1 Sink object to a futures 0.3-compatible version"] # [cfg (feature = "sink")] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] # [derive (Debug)] # [must_use = "sinks do nothing unless polled"] pub struct Compat01As03Sink < S , SinkItem > { pub (crate) inner : Spawn01 < S > , pub (crate) buffer : Option < SinkItem > , pub (crate) close_started : bool , }
    };
}

Compat01As03Sink!()