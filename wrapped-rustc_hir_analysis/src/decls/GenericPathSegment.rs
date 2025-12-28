macro_rules! GenericPathSegment {
    () => {
        # [doc = " A path segment that is semantically allowed to have generic arguments."] # [derive (Debug)] pub struct GenericPathSegment (pub DefId , pub usize) ;
    };
}

GenericPathSegment!()