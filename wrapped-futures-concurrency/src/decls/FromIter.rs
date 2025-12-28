macro_rules! FromIter {
    () => {
        # [doc = " A stream that was created from iterator."] # [pin_project] # [derive (Clone , Debug)] pub (crate) struct FromIter < I > { iter : I , }
    };
}

FromIter!()