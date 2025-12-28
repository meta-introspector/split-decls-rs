macro_rules! Header {
    () => {
        # [derive (Debug)] # [cfg (feature = "http1")] pub (super) enum Header { Token , # [cfg (any (feature = "client" , feature = "server"))] ContentLengthInvalid , # [cfg (feature = "server")] TransferEncodingInvalid , # [cfg (any (feature = "client" , feature = "server"))] TransferEncodingUnexpected , }
    };
}

Header!()