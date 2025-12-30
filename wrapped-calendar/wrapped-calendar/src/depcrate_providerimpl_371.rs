// Generated macro for impl_371 (impl)
macro_rules! Depcrate_providerimpl_371 {
() => {
// Module: crate::provider
// Provides: {"impl_371"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: Deserialize < 'de > for WeekdaySet { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { if deserializer . is_human_readable () { use core :: marker :: PhantomData ; struct Visitor < 'de > (PhantomData < & 'de () >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < 'de > { type Value = WeekdaySet ; fn expecting (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: write ! (f , "a sequence of Weekdays") } fn visit_seq < A : serde :: de :: SeqAccess < 'de > > (self , mut seq : A ,) -> Result < Self :: Value , A :: Error > { let mut set = WeekdaySet :: new (& []) ; while let Some (day) = seq . next_element :: < Weekday > () ? { set . 0 |= day . bit_value () ; } Ok (set) } } deserializer . deserialize_seq (Visitor (PhantomData)) } else { u8 :: deserialize (deserializer) . map (Self) } } }
};
}
