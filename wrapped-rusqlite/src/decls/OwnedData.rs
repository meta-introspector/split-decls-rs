macro_rules! OwnedData {
    () => {
        # [doc = " Owned serialized database"] pub struct OwnedData { ptr : NonNull < u8 > , sz : usize , }
    };
}

OwnedData!();