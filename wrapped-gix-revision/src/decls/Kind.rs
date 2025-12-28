macro_rules! Kind {
    () => {
        # [doc = " Combine one or more specs into a range of multiple."] pub trait Kind { # [doc = " Set the kind of the spec, which happens only once if it happens at all."] # [doc = " In case this method isn't called, assume `Single`."] # [doc = " Reject a kind by returning `None` to stop the parsing."] # [doc = ""] # [doc = " Note that ranges don't necessarily assure that a second specification will be parsed."] # [doc = " If `^rev` is given, this method is called with [`spec::Kind::RangeBetween`][crate::spec::Kind::RangeBetween]"] # [doc = " and no second specification is provided."] # [doc = ""] # [doc = " Note that the method can be called even if other invariants are not fulfilled, treat these as errors."] fn kind (& mut self , kind : crate :: spec :: Kind) -> Option < () > ; }
    };
}

Kind!();