// Generated macro for impl_337 (impl)
macro_rules! Depcrate_bits64_segmentationimpl_337 {
() => {
// Module: crate::bits64::segmentation
// Provides: {"impl_337"}
// Dependencies: {}
impl Descriptor64 { pub const NULL : Descriptor64 = Descriptor64 { desc32 : Descriptor :: NULL , lower : 0 , upper : 0 , } ; pub (crate) fn apply_builder_settings (& mut self , builder : & DescriptorBuilder) { self . desc32 . apply_builder_settings (builder) ; if let Some ((base , limit)) = builder . base_limit { self . set_base_limit (base , limit) } if let Some ((selector , offset)) = builder . selector_offset { self . set_selector_offset (selector , offset) } } # [doc = " Create a new segment, TSS or LDT descriptor"] # [doc = " by setting the three base and two limit fields."] pub fn set_base_limit (& mut self , base : u64 , limit : u64) { self . desc32 . set_base_limit (base as u32 , limit as u32) ; self . lower = (base >> 32) as u32 ; } # [doc = " Creates a new descriptor with selector and offset (for IDT Gate descriptors,"] # [doc = " e.g. Trap, Interrupts and Task gates)"] pub fn set_selector_offset (& mut self , selector : SegmentSelector , offset : u64) { self . desc32 . set_selector_offset (selector , offset as u32) ; self . lower = (offset >> 32) as u32 ; } # [doc = " Sets the interrupt stack table index."] # [doc = " The 3-bit IST index field that provides an offset into the IST section of the TSS."] # [doc = " Using the IST mechanism, the processor loads the value pointed by an IST pointer into the RSP."] pub fn set_ist (& mut self , index : u8) { assert ! (index <= 0b111) ; self . desc32 . upper |= index as u32 ; } }
};
}
