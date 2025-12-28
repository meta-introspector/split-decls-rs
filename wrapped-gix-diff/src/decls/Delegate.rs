macro_rules! deps {
    () => {
        Tracker!();
        Recorder!();
        Change!();
        Location!();
    };
}

macro_rules! Delegate {
    () => {
        deps!();
        struct Delegate < 'a , 'old , VisitFn , E , Objects > { src_tree : TreeRefIter < 'old > , recorder : crate :: tree :: Recorder , objects : & 'a Objects , visit : VisitFn , tracked : Option < rewrites :: Tracker < crate :: tree :: visit :: Change > > , location : Option < crate :: tree :: recorder :: Location > , err : Option < E > , }
    };
}

Delegate!();