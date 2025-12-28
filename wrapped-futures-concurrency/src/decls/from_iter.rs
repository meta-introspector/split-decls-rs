macro_rules! deps {
    () => {
        FromIter!();
    };
}

macro_rules! from_iter {
    () => {
        deps!();
        # [doc = " Converts an iterator into a stream."] pub (crate) fn from_iter < I : IntoIterator > (iter : I) -> FromIter < I :: IntoIter > { FromIter { iter : iter . into_iter () , } }
    };
}

from_iter!()