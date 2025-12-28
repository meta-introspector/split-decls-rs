macro_rules! deps {
    () => {
        Difference!();
        HashSet!();
    };
}

macro_rules! SymmetricDifference {
    () => {
        deps!();
        # [doc = " A lazy iterator producing elements in the symmetric difference of `HashSet`s."] # [doc = ""] # [doc = " This `struct` is created by the [`symmetric_difference`] method on"] # [doc = " [`HashSet`]. See its documentation for more."] # [doc = ""] # [doc = " [`HashSet`]: struct.HashSet.html"] # [doc = " [`symmetric_difference`]: struct.HashSet.html#method.symmetric_difference"] pub struct SymmetricDifference < 'a , T , S , A : Allocator = Global > { iter : Chain < Difference < 'a , T , S , A > , Difference < 'a , T , S , A > > , }
    };
}

SymmetricDifference!()