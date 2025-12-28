macro_rules! deps {
    () => {
        ResSender!();
    };
}

macro_rules! KeysAndSender {
    () => {
        deps!();
        type KeysAndSender < K , T > = (HashSet < K > , Vec < (HashSet < K > , ResSender < K , T >) >) ;
    };
}

KeysAndSender!()