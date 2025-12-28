macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! lookup {
    () => {
        deps!();
        # [doc = ""] pub mod lookup { use crate :: loose ; # [doc = " Returned by [`Handle::lookup_prefix()`][crate::store::Handle::lookup_prefix()]"] # [derive (thiserror :: Error , Debug)] # [allow (missing_docs)] pub enum Error { # [error ("An error occurred looking up a prefix which requires iteration")] LooseWalkDir (# [from] loose :: iter :: Error) , # [error (transparent)] LoadIndex (# [from] crate :: store :: load_index :: Error) , } # [doc = " A way to indicate if a lookup, despite successful, was ambiguous or yielded exactly"] # [doc = " one result in the particular index."] pub type Outcome = Result < gix_hash :: ObjectId , () > ; }
    };
}

lookup!();