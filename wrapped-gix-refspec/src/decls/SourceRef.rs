macro_rules! SourceRef {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] # [doc = " The source (or left-hand) side of a mapping."] pub enum SourceRef < 'a > { # [doc = " A full reference name, which is expected to be valid."] # [doc = ""] # [doc = " Validity, however, is not enforced here."] FullName (Cow < 'a , BStr >) , # [doc = " The name of an object that is expected to exist on the remote side."] # [doc = " Note that it might not be advertised by the remote but part of the object graph,"] # [doc = " and thus gets sent in the pack. The server is expected to fail unless the desired"] # [doc = " object is present but at some time it is merely a request by the user."] ObjectId (gix_hash :: ObjectId) , }
    };
}

SourceRef!();