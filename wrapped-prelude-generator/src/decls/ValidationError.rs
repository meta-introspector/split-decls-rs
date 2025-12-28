macro_rules! ValidationError {
    () => {
        # [derive (Debug)] pub enum ValidationError { DependencyResolutionError (String) , MissingCargoInfo (String) , UnresolvedDependency (ResolvedDependency) , }
    };
}

ValidationError!()