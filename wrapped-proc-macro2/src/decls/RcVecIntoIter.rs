macro_rules! RcVecIntoIter {
    () => {
        # [derive (Clone)] pub (crate) struct RcVecIntoIter < T > { inner : vec :: IntoIter < T > , }
    };
}

RcVecIntoIter!()