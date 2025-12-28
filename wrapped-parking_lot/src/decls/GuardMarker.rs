macro_rules! GuardMarker {
    () => {
        # [cfg (not (feature = "send_guard"))] type GuardMarker = lock_api :: GuardNoSend ;
    };
}

GuardMarker!()