// Generated macro for impl_779 (impl)
macro_rules! Depcrateimpl_779 {
() => {
// Module: crate
// Provides: {"impl_779"}
// Dependencies: {}
# [cfg (has_num_saturating)] impl < T : Num > Num for core :: num :: Saturating < T > where core :: num :: Saturating < T > : NumOps , { type FromStrRadixErr = T :: FromStrRadixErr ; fn from_str_radix (str : & str , radix : u32) -> Result < Self , Self :: FromStrRadixErr > { T :: from_str_radix (str , radix) . map (core :: num :: Saturating) } }
};
}
