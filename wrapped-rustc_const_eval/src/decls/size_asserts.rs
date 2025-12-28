macro_rules! deps {
    () => {
        Place!();
        PlaceTy!();
        MemPlace!();
        MemPlaceMeta!();
        MPlaceTy!();
    };
}

macro_rules! size_asserts {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] mod size_asserts { use rustc_data_structures :: static_assert_size ; use super :: * ; static_assert_size ! (MPlaceTy <'_ >, 64) ; static_assert_size ! (MemPlace , 48) ; static_assert_size ! (MemPlaceMeta , 24) ; static_assert_size ! (Place , 48) ; static_assert_size ! (PlaceTy <'_ >, 64) ; }
    };
}

size_asserts!()