macro_rules! deps {
    () => {
        TrailerRef!();
        CommitRef!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [doc = " Convenience methods"] impl < 'a > CommitRef < 'a > { # [doc = " Get an iterator over all `Signed-off-by` trailers in the commit message."] # [doc = " This is useful for finding who signed off on the commit."] pub fn signed_off_by_trailers (& self) -> impl Iterator < Item = body :: TrailerRef < 'a > > { self . message_trailers () . signed_off_by () } # [doc = " Get an iterator over `Co-authored-by` trailers in the commit message."] # [doc = " This is useful for squashed commits that contain multiple authors."] pub fn co_authored_by_trailers (& self) -> impl Iterator < Item = body :: TrailerRef < 'a > > { self . message_trailers () . co_authored_by () } # [doc = " Get all authors mentioned in `Signed-off-by` and `Co-authored-by` trailers."] # [doc = " This is useful for squashed commits that contain multiple authors."] # [doc = " Returns a Vec of author strings that can include both signers and co-authors."] pub fn author_trailers (& self) -> impl Iterator < Item = body :: TrailerRef < 'a > > { self . message_trailers () . authors () } # [doc = " Get an iterator over all attribution-related trailers"] # [doc = " (`Signed-off-by,` `Co-authored-by`, `Acked-by`, `Reviewed-by`, `Tested-by`)."] # [doc = " This provides a comprehensive view of everyone who contributed to or reviewed the commit."] # [doc = " Note that the same name may occur multiple times, it's not a unified list."] pub fn attribution_trailers (& self) -> impl Iterator < Item = body :: TrailerRef < 'a > > { self . message_trailers () . attributions () } }
    };
}

impl_20!()