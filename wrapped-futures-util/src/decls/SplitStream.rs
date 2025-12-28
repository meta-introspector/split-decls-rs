macro_rules! deps {
    () => {
        BiLock!();
    };
}

macro_rules! SplitStream {
    () => {
        deps!();
        # [doc = " A `Stream` part of the split pair"] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] pub struct SplitStream < S > (BiLock < S >) ;
    };
}

SplitStream!();