// Generated macro for impl_1721 (impl)
macro_rules! Depcrate_segmentationimpl_1721 {
() => {
// Module: crate::segmentation
// Provides: {"impl_1721"}
// Dependencies: {}
impl BuildDescriptor < Descriptor > for DescriptorBuilder { fn finish (& self) -> Descriptor { let mut desc : Descriptor = Default :: default () ; desc . apply_builder_settings (self) ; let typ = match self . typ { Some (DescriptorType :: System64 (_)) => { panic ! ("You shall not use 64-bit types on 32-bit descriptor.") } Some (DescriptorType :: System32 (typ)) => typ as u8 , Some (DescriptorType :: Data (typ)) => { desc . set_s () ; typ as u8 } Some (DescriptorType :: Code (typ)) => { desc . set_s () ; typ as u8 } None => unreachable ! ("Type not set, this is a library bug in x86.") , } ; desc . set_type (typ) ; desc } }
};
}
