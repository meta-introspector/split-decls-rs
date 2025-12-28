macro_rules! Interleave {
    () => {
        # [doc = " An iterator adaptor that alternates elements from two iterators until both"] # [doc = " run out."] # [doc = ""] # [doc = " This iterator is *fused*."] # [doc = ""] # [doc = " See [`.interleave()`](crate::Itertools::interleave) for more information."] # [derive (Clone , Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Interleave < I , J > { i : Fuse < I > , j : Fuse < J > , next_coming_from_j : bool , }
    };
}

Interleave!()