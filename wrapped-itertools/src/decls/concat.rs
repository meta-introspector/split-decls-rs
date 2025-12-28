macro_rules! concat {
    () => {
        # [doc = " Combine all an iterator's elements into one element by using [`Extend`]."] # [doc = ""] # [doc = " [`IntoIterator`]-enabled version of [`Itertools::concat`](crate::Itertools::concat)."] # [doc = ""] # [doc = " This combinator will extend the first item with each of the rest of the"] # [doc = " items of the iterator. If the iterator is empty, the default value of"] # [doc = " `I::Item` is returned."] # [doc = ""] # [doc = " ```rust"] # [doc = " use itertools::concat;"] # [doc = ""] # [doc = " let input = vec![vec![1], vec![2, 3], vec![4, 5, 6]];"] # [doc = " assert_eq!(concat(input), vec![1, 2, 3, 4, 5, 6]);"] # [doc = " ```"] pub fn concat < I > (iterable : I) -> I :: Item where I : IntoIterator , I :: Item : Extend < < < I as IntoIterator > :: Item as IntoIterator > :: Item > + IntoIterator + Default , { iterable . into_iter () . reduce (| mut a , b | { a . extend (b) ; a }) . unwrap_or_default () }
    };
}

concat!()