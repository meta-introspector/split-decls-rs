macro_rules! deps {
    () => {
        VersionVec!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl cmp :: PartialOrd for VersionVec { fn partial_cmp (& self , other : & VersionVec) -> Option < cmp :: Ordering > { use cmp :: Ordering :: * ; let mut ret = Equal ; for i in 0 .. MAX_THREADS { let a = self . versions [i] ; let b = other . versions [i] ; match a . cmp (& b) { Equal => { } Less if ret == Greater => return None , Greater if ret == Less => return None , ordering => ret = ordering , } } Some (ret) } }
    };
}

impl_178!()