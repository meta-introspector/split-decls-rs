macro_rules! deps {
    () => {
        Arc!();
        Encoder!();
        ObjectIdentifier!();
    };
}

macro_rules! Parser {
    () => {
        deps!();
        # [doc = " Const-friendly OID string parser."] # [doc = ""] # [doc = " Parses an OID from the dotted string representation."] # [derive (Debug)] pub (crate) struct Parser { # [doc = " Current arc in progress"] current_arc : Option < Arc > , # [doc = " BER/DER encoder"] encoder : Encoder < { ObjectIdentifier :: MAX_SIZE } > , }
    };
}

Parser!()