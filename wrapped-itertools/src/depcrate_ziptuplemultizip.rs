// Generated macro for multizip (function)
macro_rules! Depcrate_ziptuplemultizip {
() => {
// Module: crate::ziptuple
// Provides: {"multizip"}
// Dependencies: {}
# [doc = " An iterator that generalizes `.zip()` and allows running multiple iterators in lockstep."] # [doc = ""] # [doc = " The iterator `Zip<(I, J, ..., M)>` is formed from a tuple of iterators (or values that"] # [doc = " implement [`IntoIterator`]) and yields elements"] # [doc = " until any of the subiterators yields `None`."] # [doc = ""] # [doc = " The iterator element type is a tuple like `(A, B, ..., E)` where `A` to `E` are the"] # [doc = " element types of the subiterator."] # [doc = ""] # [doc = " **Note:** The result of this function is a value of a named type (`Zip<(I, J,"] # [doc = " ..)>` of each component iterator `I, J, ...`) if each component iterator is"] # [doc = " nameable."] # [doc = ""] # [doc = " Prefer [`izip!()`](crate::izip) over `multizip` for the performance benefits of using the"] # [doc = " standard library `.zip()`. Prefer `multizip` if a nameable type is needed."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::multizip;"] # [doc = ""] # [doc = " // iterate over three sequences side-by-side"] # [doc = " let mut results = [0, 0, 0, 0];"] # [doc = " let inputs = [3, 7, 9, 6];"] # [doc = ""] # [doc = " for (r, index, input) in multizip((&mut results, 0..10, &inputs)) {"] # [doc = "     *r = index * 10 + input;"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(results, [0 + 3, 10 + 7, 29, 36]);"] # [doc = " ```"] pub fn multizip < T , U > (t : U) -> Zip < T > where Zip < T > : From < U > + Iterator , { Zip :: from (t) }
};
}
