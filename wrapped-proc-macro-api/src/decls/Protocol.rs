macro_rules! Protocol {
    () => {
        # [derive (Debug)] enum Protocol { LegacyJson { mode : SpanMode } , }
    };
}

Protocol!()