// Generated macro for impl_273 (impl)
macro_rules! Depcrate_value_rawimpl_273 {
() => {
// Module: crate::value::raw
// Provides: {"impl_273"}
// Dependencies: {}
# [allow (unsafe_code)] impl RawValue { fn from_borrowed_str (ron : & str) -> & Self { unsafe { & * (ron as * const str as * const RawValue) } } fn from_boxed_str (ron : Box < str >) -> Box < Self > { unsafe { core :: mem :: transmute :: < Box < str > , Box < RawValue > > (ron) } } fn into_boxed_str (raw_value : Box < Self >) -> Box < str > { unsafe { core :: mem :: transmute :: < Box < RawValue > , Box < str > > (raw_value) } } # [allow (clippy :: expect_used)] fn trim_range (ron : & str) -> Range < usize > { fn trim_range_inner (ron : & str) -> Result < Range < usize > , Error > { let mut deserializer = crate :: Deserializer :: from_str (ron) . map_err (Error :: from) ? ; deserializer . parser . skip_ws () ? ; let start_offset = ron . len () - deserializer . parser . src () . len () ; let _ = serde :: de :: IgnoredAny :: deserialize (& mut deserializer) ? ; deserializer . parser . skip_ws () ? ; let end_offset = deserializer . parser . pre_ws_src () . len () ; Ok (start_offset .. (ron . len () - end_offset)) } trim_range_inner (ron) . expect ("RawValue must contain valid ron") } }
};
}
