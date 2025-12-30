// Generated macro for impl_81 (impl)
macro_rules! Depcrate_datetimeimpl_81 {
() => {
// Module: crate::datetime
// Provides: {"impl_81"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < 'a > arbitrary :: Arbitrary < 'a > for DateTime { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { Self :: from_unix_duration (Duration :: new (u . int_in_range (0 ..= MAX_UNIX_DURATION . as_secs () . saturating_sub (1)) ? , u . int_in_range (0 ..= 999_999_999) ? ,)) . map_err (| _ | arbitrary :: Error :: IncorrectFormat) } fn size_hint (depth : usize) -> (usize , Option < usize >) { arbitrary :: size_hint :: and (u64 :: size_hint (depth) , u32 :: size_hint (depth)) } }
};
}
