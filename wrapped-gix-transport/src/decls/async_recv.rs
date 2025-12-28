macro_rules! deps {
    () => {
        Protocol!();
        Error!();
        ReadlineBufRead!();
        Capabilities!();
    };
}

macro_rules! async_recv {
    () => {
        deps!();
        # [doc = ""] # [cfg (feature = "async-client")] # [allow (missing_docs)] pub mod async_recv { use bstr :: ByteVec ; use futures_io :: AsyncRead ; use crate :: { client :: { self , async_io :: ReadlineBufRead , Capabilities } , packetline :: async_io :: StreamingPeekableIter , Protocol , } ; # [doc = " The information provided by the server upon first connection."] pub struct Handshake < 'a > { # [doc = " The [`Capabilities`] the remote advertised."] pub capabilities : Capabilities , # [doc = " The remote refs as an [`AsyncBufRead`]."] # [doc = ""] # [doc = " This is `Some` only when protocol v1 is used. The [`AsyncRead`] must be exhausted by"] # [doc = " the caller."] pub refs : Option < Box < dyn ReadlineBufRead + Unpin + 'a > > , # [doc = " The [`Protocol`] the remote advertised."] pub protocol : Protocol , } impl Handshake < '_ > { # [doc = " Read the capabilities and version advertisement from the given packetline reader."] # [doc = ""] # [doc = " If [`Protocol::V1`] was requested, or the remote decided to downgrade, the remote refs"] # [doc = " advertisement will also be included in the [`Handshake`]."] pub async fn from_lines_with_version_detection < T : AsyncRead + Unpin > (rd : & mut StreamingPeekableIter < T > ,) -> Result < Handshake < '_ > , client :: Error > { rd . fail_on_err_lines (true) ; Ok (match rd . peek_line () . await { Some (line) => { let line = line ? ? . as_text () . ok_or (client :: Error :: ExpectedLine ("text")) ? ; let version = Capabilities :: extract_protocol (line) ? ; match version { Protocol :: V0 => unreachable ! ("already handled in `None` case") , Protocol :: V1 => { let (capabilities , delimiter_position) = Capabilities :: from_bytes (line . 0) ? ; rd . peek_buffer_replace_and_truncate (delimiter_position , b'\n') ; Handshake { capabilities , refs : Some (Box :: new (rd . as_read ())) , protocol : Protocol :: V1 , } } Protocol :: V2 => Handshake { capabilities : { let mut rd = rd . as_read () ; let mut buf = Vec :: new () ; while let Some (line) = rd . read_data_line () . await { let line = line ? ? ; match line . as_bstr () { Some (line) => { buf . push_str (line) ; if buf . last () != Some (& b'\n') { buf . push (b'\n') ; } } None => break , } } Capabilities :: from_lines (buf . into ()) ? } , refs : None , protocol : Protocol :: V2 , } , } } None => Handshake { capabilities : Capabilities :: default () , refs : Some (Box :: new (rd . as_read ())) , protocol : Protocol :: V0 , } , }) } } }
    };
}

async_recv!();