macro_rules! private_iter_index {
    () => {
        mod private_iter_index { use core :: ops ; pub trait Sealed { } impl Sealed for ops :: Range < usize > { } impl Sealed for ops :: RangeInclusive < usize > { } impl Sealed for ops :: RangeTo < usize > { } impl Sealed for ops :: RangeToInclusive < usize > { } impl Sealed for ops :: RangeFrom < usize > { } impl Sealed for ops :: RangeFull { } }
    };
}

private_iter_index!();