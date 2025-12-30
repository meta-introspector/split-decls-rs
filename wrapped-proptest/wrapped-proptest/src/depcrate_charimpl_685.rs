// Generated macro for impl_685 (impl)
macro_rules! Depcrate_charimpl_685 {
() => {
// Module: crate::char
// Provides: {"impl_685"}
// Dependencies: {}
impl < 'a > Strategy for CharStrategy < 'a > { type Tree = CharValueTree ; type Value = char ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let (base , offset) = select_range_index (runner . rng () , & self . special , & self . preferred , & self . ranges ,) ; let start = base + offset ; let bottom = if start >= '¡' as u32 && base < '¡' as u32 { '¡' as u32 } else if start >= 'a' as u32 && base < 'a' as u32 { 'a' as u32 } else if start >= 'A' as u32 && base < 'A' as u32 { 'A' as u32 } else if start >= '0' as u32 && base < '0' as u32 { '0' as u32 } else if start >= ' ' as u32 && base < ' ' as u32 { ' ' as u32 } else { base } ; Ok (CharValueTree { value : num :: u32 :: BinarySearch :: new_above (bottom , start) , }) } }
};
}
