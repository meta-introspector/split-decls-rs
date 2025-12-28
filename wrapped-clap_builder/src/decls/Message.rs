macro_rules! deps {
    () => {
        StyledStr!();
    };
}

macro_rules! Message {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) enum Message { Raw (String) , Formatted (StyledStr) , }
    };
}

Message!()