macro_rules! deps {
    () => {
        Captures!();
        GroupInfoPatternNames!();
    };
}

macro_rules! CapturesPatternIter {
    () => {
        deps!();
        # [doc = " An iterator over all capturing groups in a `Captures` value."] # [doc = ""] # [doc = " This iterator includes capturing groups that did not participate in a"] # [doc = " match. See the [`Captures::iter`] method documentation for more details"] # [doc = " and examples."] # [doc = ""] # [doc = " The lifetime parameter `'a` refers to the lifetime of the underlying"] # [doc = " `Captures` value."] # [derive (Clone , Debug)] pub struct CapturesPatternIter < 'a > { caps : & 'a Captures , names : core :: iter :: Enumerate < GroupInfoPatternNames < 'a > > , }
    };
}

CapturesPatternIter!();