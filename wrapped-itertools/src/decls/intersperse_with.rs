macro_rules! deps {
    () => {
        IntersperseWith!();
    };
}

macro_rules! intersperse_with {
    () => {
        deps!();
        # [doc = " Create a new `IntersperseWith` iterator"] pub fn intersperse_with < I , ElemF > (iter : I , elt : ElemF) -> IntersperseWith < I , ElemF > where I : Iterator , { IntersperseWith { peek : None , iter : iter . fuse () , element : elt , } }
    };
}

intersperse_with!();