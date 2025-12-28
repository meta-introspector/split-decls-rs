macro_rules! deps {
    () => {
        IntersperseWith!();
        IntersperseElementSimple!();
    };
}

macro_rules! Intersperse {
    () => {
        deps!();
        # [doc = " An iterator adaptor to insert a particular value"] # [doc = " between each element of the adapted iterator."] # [doc = ""] # [doc = " Iterator element type is `I::Item`"] # [doc = ""] # [doc = " This iterator is *fused*."] # [doc = ""] # [doc = " See [`.intersperse()`](crate::Itertools::intersperse) for more information."] pub type Intersperse < I > = IntersperseWith < I , IntersperseElementSimple < < I as Iterator > :: Item > > ;
    };
}

Intersperse!();