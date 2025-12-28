macro_rules! deps {
    () => {
        Rng!();
    };
}

macro_rules! index {
    () => {
        deps!();
        # [doc = " Low-level API for sampling indices"] pub mod index { use crate :: Rng ; # [cfg (feature = "alloc")] # [doc (inline)] pub use super :: index_ :: * ; # [doc = " Randomly sample exactly `N` distinct indices from `0..len`, and"] # [doc = " return them in random order (fully shuffled)."] # [doc = ""] # [doc = " This is implemented via Floyd's algorithm. Time complexity is `O(N^2)`"] # [doc = " and memory complexity is `O(N)`."] # [doc = ""] # [doc = " Returns `None` if (and only if) `N > len`."] pub fn sample_array < R , const N : usize > (rng : & mut R , len : usize) -> Option < [usize ; N] > where R : Rng + ? Sized , { if N > len { return None ; } let mut indices = [0 ; N] ; for (i , j) in (len - N .. len) . enumerate () { let t = rng . random_range (.. j + 1) ; if let Some (pos) = indices [0 .. i] . iter () . position (| & x | x == t) { indices [pos] = j ; } indices [i] = t ; } Some (indices) } }
    };
}

index!();