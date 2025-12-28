macro_rules! intersects {
    () => {
        # [doc = " Returns true if and only if the given ranges intersect."] fn intersects (r1 : Utf8Range , r2 : Utf8Range) -> bool { ! (r1 . end < r2 . start || r2 . end < r1 . start) }
    };
}

intersects!();