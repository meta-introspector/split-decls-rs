macro_rules! deps {
    () => {
        Message!();
    };
}

macro_rules! MessageIterator {
    () => {
        deps!();
        # [doc = " An iterator of Message."] type MessageIterator < R > = serde_json :: StreamDeserializer < 'static , serde_json :: de :: IoRead < R > , Message > ;
    };
}

MessageIterator!()