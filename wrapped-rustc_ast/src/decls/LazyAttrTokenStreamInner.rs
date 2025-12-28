macro_rules! deps {
    () => {
        NodeReplacement!();
        AttrTokenStream!();
        Token!();
        Spacing!();
        TokenCursor!();
    };
}

macro_rules! LazyAttrTokenStreamInner {
    () => {
        deps!();
        enum LazyAttrTokenStreamInner { Direct (AttrTokenStream) , Pending { start_token : (Token , Spacing) , cursor_snapshot : TokenCursor , num_calls : u32 , break_last_token : u32 , node_replacements : ThinVec < NodeReplacement > , } , }
    };
}

LazyAttrTokenStreamInner!();