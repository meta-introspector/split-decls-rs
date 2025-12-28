macro_rules! Context {
    () => {
        # [doc = " For use in [`RefMap::fetch()`] and [`fetch`](crate::fetch())."] # [cfg (feature = "handshake")] pub struct Context < 'a , T > { # [doc = " The outcome of the handshake performed with the remote."] # [doc = ""] # [doc = " Note that it's mutable as depending on the protocol, it may contain refs that have been sent unconditionally."] pub handshake : & 'a mut crate :: Handshake , # [doc = " The transport to use when making an `ls-refs` or `fetch` call."] # [doc = ""] # [doc = " This is always done if the underlying protocol is V2, which is implied by the absence of refs in the `handshake` outcome."] pub transport : & 'a mut T , # [doc = " How to self-identify during the `ls-refs` call in [`RefMap::fetch()`] or the `fetch` call in [`fetch()`](crate::fetch())."] # [doc = ""] # [doc = " This could be read from the `gitoxide.userAgent` configuration variable."] pub user_agent : (& 'static str , Option < std :: borrow :: Cow < 'static , str > >) , # [doc = " If `true`, output all packetlines using the `gix-trace` machinery."] pub trace_packetlines : bool , }
    };
}

Context!()