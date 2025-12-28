macro_rules! deps {
    () => {
        Parse!();
        Header!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        # [cfg (feature = "http1")] impl Parse { # [cfg (any (feature = "client" , feature = "server"))] pub (crate) fn content_length_invalid () -> Self { Parse :: Header (Header :: ContentLengthInvalid) } # [cfg (feature = "server")] pub (crate) fn transfer_encoding_invalid () -> Self { Parse :: Header (Header :: TransferEncodingInvalid) } # [cfg (any (feature = "client" , feature = "server"))] pub (crate) fn transfer_encoding_unexpected () -> Self { Parse :: Header (Header :: TransferEncodingUnexpected) } }
    };
}

impl_114!()