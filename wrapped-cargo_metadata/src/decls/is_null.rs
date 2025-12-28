macro_rules! is_null {
    () => {
        # [doc = " Helpers for default metadata fields"] fn is_null (value : & serde_json :: Value) -> bool { matches ! (value , serde_json :: Value :: Null) }
    };
}

is_null!()