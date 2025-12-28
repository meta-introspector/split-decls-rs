macro_rules! deps {
    () => {
        Capabilities!();
        Protocol!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " The error used in [`Capabilities::from_bytes()`] and [`Capabilities::from_lines()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Capabilities were missing entirely as there was no 0 byte")] MissingDelimitingNullByte , # [error ("there was not a single capability behind the delimiter")] NoCapabilities , # [error ("a version line was expected, but none was retrieved")] MissingVersionLine , # [error ("expected 'version X', got {0:?}")] MalformattedVersionLine (BString) , # [error ("Got unsupported version {actual:?}, expected {}" , * desired as u8)] UnsupportedVersion { desired : Protocol , actual : BString } , # [error ("An IO error occurred while reading V2 lines")] Io (# [from] std :: io :: Error) , }
    };
}

Error!();