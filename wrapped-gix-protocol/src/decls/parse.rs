macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! parse {
    () => {
        deps!();
        # [doc = ""] pub mod parse { use bstr :: BString ; # [doc = " The error returned when parsing References/refs from the server response."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Io (# [from] std :: io :: Error) , # [error (transparent)] DecodePacketline (# [from] gix_transport :: packetline :: decode :: Error) , # [error (transparent)] Id (# [from] gix_hash :: decode :: Error) , # [error ("{symref:?} could not be parsed. A symref is expected to look like <NAME>:<target>.")] MalformedSymref { symref : BString } , # [error ("{0:?} could not be parsed. A V1 ref line should be '<hex-hash> <path>'.")] MalformedV1RefLine (BString) , # [error ("{0:?} could not be parsed. A V2 ref line should be '<hex-hash> <path>[ (peeled|symref-target):<value>'.")] MalformedV2RefLine (BString) , # [error ("The ref attribute {attribute:?} is unknown. Found in line {line:?}")] UnknownAttribute { attribute : BString , line : BString } , # [error ("{message}")] InvariantViolation { message : & 'static str } , } }
    };
}

parse!()