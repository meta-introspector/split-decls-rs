macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! Revspec {
    () => {
        deps!();
        # [doc = " A revspec represents a range of revisions within a repository."] pub struct Revspec < 'repo > { from : Option < Object < 'repo > > , to : Option < Object < 'repo > > , mode : RevparseMode , }
    };
}

Revspec!();