macro_rules! deps {
    () => {
        Zip!();
    };
}

macro_rules! zip {
    () => {
        deps!();
        # [doc = " Converts the arguments to iterators and zips them."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Iterator::zip`]."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::zip;"] # [doc = ""] # [doc = " let mut result: Vec<(i32, char)> = Vec::new();"] # [doc = ""] # [doc = " for (a, b) in zip(&[1, 2, 3, 4, 5], &['a', 'b', 'c']) {"] # [doc = "     result.push((*a, *b));"] # [doc = " }"] # [doc = " assert_eq!(result, vec![(1, 'a'), (2, 'b'), (3, 'c')]);"] # [doc = " ```"] # [deprecated (note = "Use [std::iter::zip](https://doc.rust-lang.org/std/iter/fn.zip.html) instead" , since = "0.10.4")] pub fn zip < I , J > (i : I , j : J) -> Zip < I :: IntoIter , J :: IntoIter > where I : IntoIterator , J : IntoIterator , { i . into_iter () . zip (j) }
    };
}

zip!()