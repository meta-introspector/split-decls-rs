macro_rules! Names {
    () => {
        # [doc = " A type-definition for a sorted list of unvalidated remote names - they have been read straight from the configuration."] pub type Names < 'a > = BTreeSet < Cow < 'a , BStr > > ;
    };
}

Names!();