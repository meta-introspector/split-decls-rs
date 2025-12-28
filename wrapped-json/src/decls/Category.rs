macro_rules! Category {
    () => {
        # [doc = " Categorizes the cause of a `serde_json::Error`."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum Category { # [doc = " The error was caused by a failure to read or write bytes on an I/O"] # [doc = " stream."] Io , # [doc = " The error was caused by input that was not syntactically valid JSON."] Syntax , # [doc = " The error was caused by input data that was semantically incorrect."] # [doc = ""] # [doc = " For example, JSON containing a number is semantically incorrect when the"] # [doc = " type being deserialized into holds a String."] Data , # [doc = " The error was caused by prematurely reaching the end of the input data."] # [doc = ""] # [doc = " Callers that process streaming input may be interested in retrying the"] # [doc = " deserialization once more data is available."] Eof , }
    };
}

Category!();