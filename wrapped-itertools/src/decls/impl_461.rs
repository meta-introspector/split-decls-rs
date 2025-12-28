macro_rules! deps {
    () => {
        RcIter!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        # [doc = " Return an iterator from `&RcIter<I>` (by simply cloning it)."] impl < I > IntoIterator for & RcIter < I > where I : Iterator , { type Item = I :: Item ; type IntoIter = RcIter < I > ; fn into_iter (self) -> RcIter < I > { self . clone () } }
    };
}

impl_461!();