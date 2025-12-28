macro_rules! LifetimeElisionHints {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub enum LifetimeElisionHints { Always , SkipTrivial , Never , }
    };
}

LifetimeElisionHints!()