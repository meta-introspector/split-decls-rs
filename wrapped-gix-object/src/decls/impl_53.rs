macro_rules! deps {
    () => {
        CommitRef!();
        Error!();
        Commit!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl TryFrom < CommitRef < '_ > > for Commit { type Error = crate :: decode :: Error ; fn try_from (other : CommitRef < '_ >) -> Result < Commit , Self :: Error > { let CommitRef { tree , parents , author , committer , encoding , message , extra_headers , } = other ; let untrimmed_author = parse_signature (author) ? ; let untrimmed_committer = parse_signature (committer) ? ; Ok (Commit { tree : gix_hash :: ObjectId :: from_hex (tree) . expect ("prior parser validation") , parents : parents . iter () . map (| parent | gix_hash :: ObjectId :: from_hex (parent) . expect ("prior parser validation")) . collect () , author : untrimmed_author . into () , committer : untrimmed_committer . into () , encoding : encoding . map (ToOwned :: to_owned) , message : message . to_owned () , extra_headers : extra_headers . into_iter () . map (| (k , v) | (k . into () , v . into_owned ())) . collect () , }) } }
    };
}

impl_53!();