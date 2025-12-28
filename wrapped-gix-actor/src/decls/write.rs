macro_rules! deps {
    () => {
        Signature!();
        SignatureRef!();
    };
}

macro_rules! write {
    () => {
        deps!();
        pub (crate) mod write { use bstr :: { BStr , ByteSlice } ; use gix_date :: parse :: TimeBuf ; use crate :: { Signature , SignatureRef } ; # [doc = " The Error produced by [`Signature::write_to()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub (crate) enum Error { # [error (r"Signature name or email must not contain '<', '>' or \n")] IllegalCharacter , } impl From < Error > for std :: io :: Error { fn from (err : Error) -> Self { std :: io :: Error :: other (err) } } # [doc = " Output"] impl Signature { # [doc = " Serialize this instance to `out` in the git serialization format for actors."] pub fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { let mut buf = TimeBuf :: default () ; self . to_ref (& mut buf) . write_to (out) } # [doc = " Computes the number of bytes necessary to serialize this signature"] pub fn size (& self) -> usize { self . name . len () + 2 + self . email . len () + 2 + self . time . size () } } impl SignatureRef < '_ > { # [doc = " Serialize this instance to `out` in the git serialization format for actors."] pub fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { out . write_all (validated_token (self . name) ?) ? ; out . write_all (b" ") ? ; out . write_all (b"<") ? ; out . write_all (validated_token (self . email) ?) ? ; out . write_all (b"> ") ? ; out . write_all (validated_token (self . time . into ()) ?) } # [doc = " Computes the number of bytes necessary to serialize this signature"] pub fn size (& self) -> usize { self . name . len () + 2 + self . email . len () + 2 + self . time . len () } } pub (crate) fn validated_token (name : & BStr) -> Result < & BStr , Error > { if name . find_byteset (b"<>\n") . is_some () { return Err (Error :: IllegalCharacter) ; } Ok (name) } }
    };
}

write!()