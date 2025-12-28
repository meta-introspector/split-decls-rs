macro_rules! PrivateMarker {
    () => {
        # [allow (missing_debug_implementations)] pub struct PrivateMarker ;
    };
}

PrivateMarker!()