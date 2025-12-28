macro_rules! deps {
    () => {
        ZipEq!();
    };
}

macro_rules! zip_eq {
    () => {
        deps!();
        # [doc = " Zips two iterators but **panics** if they are not of the same length."] # [doc = ""] # [doc = " [`IntoIterator`] enabled version of [`Itertools::zip_eq`](crate::Itertools::zip_eq)."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::zip_eq;"] # [doc = ""] # [doc = " let data = [1, 2, 3, 4, 5];"] # [doc = " for (a, b) in zip_eq(&data[..data.len() - 1], &data[1..]) {"] # [doc = "     /* loop body */"] # [doc = "     # let _ = (a, b);"] # [doc = " }"] # [doc = " ```"] pub fn zip_eq < I , J > (i : I , j : J) -> ZipEq < I :: IntoIter , J :: IntoIter > where I : IntoIterator , J : IntoIterator , { ZipEq { a : i . into_iter () , b : j . into_iter () , } }
    };
}

zip_eq!()