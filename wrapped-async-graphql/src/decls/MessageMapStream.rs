macro_rules! deps {
    () => {
        ClientMessage!();
        Result!();
    };
}

macro_rules! MessageMapStream {
    () => {
        deps!();
        type MessageMapStream < S > = futures_util :: stream :: Map < S , fn (< S as Stream > :: Item) -> serde_json :: Result < ClientMessage > > ;
    };
}

MessageMapStream!();