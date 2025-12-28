macro_rules! deps {
    () => {
        UnknownCryptoError!();
    };
}

macro_rules! impl_try_from_trait {
    () => {
        deps!();
        # [doc = " Macro that implements `TryFrom<&[u8]>` on an object called `$name` that"] # [doc = " implements the method `from_slice`."] macro_rules ! impl_try_from_trait (($ name : ident) => (# [doc = " Delegates to `from_slice` implementation"] impl TryFrom <& [u8] > for $ name { type Error = UnknownCryptoError ; fn try_from (slice : & [u8]) -> Result < Self , Self :: Error > { Self :: from_slice (slice) } })) ;
    };
}

impl_try_from_trait!();