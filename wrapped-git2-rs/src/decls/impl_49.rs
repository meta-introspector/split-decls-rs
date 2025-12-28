macro_rules! deps {
    () => {
        AttrValue!();
        Note!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        # [doc = " Compare two [`AttrValue`]s."] # [doc = ""] # [doc = " Note that this implementation does not differentiate between [`AttrValue::String`] and"] # [doc = " [`AttrValue::Bytes`]."] impl PartialEq for AttrValue < '_ > { fn eq (& self , other : & AttrValue < '_ >) -> bool { match (self , other) { (Self :: True , AttrValue :: True) | (Self :: False , AttrValue :: False) | (Self :: Unspecified , AttrValue :: Unspecified) => true , (AttrValue :: String (string) , AttrValue :: Bytes (bytes)) | (AttrValue :: Bytes (bytes) , AttrValue :: String (string)) => string . as_bytes () == * bytes , (AttrValue :: String (left) , AttrValue :: String (right)) => left == right , (AttrValue :: Bytes (left) , AttrValue :: Bytes (right)) => left == right , _ => false , } } }
    };
}

impl_49!();